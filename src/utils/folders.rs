use std::path::PathBuf;
use std::{fs, io};
use egui::{Button, RichText, Color32, Stroke, Sense, PointerButton, Pos2};

use utils::files::*;

use crate::app::App;
use crate::structs::file_data::FileData;
use crate::utils;


pub fn ui_folders(ui: &mut egui::Ui, _self: &mut App, index: &i32, curr_path: &str, screen_height: &f32) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(200.0, *screen_height - 100.0),
        Sense::click(),
    );

    let painter = ui.painter_at(rect).to_owned();

    painter.rect_filled(rect, 0.0, Color32::TRANSPARENT);

    ui.allocate_ui_at_rect(rect, |ui| {
        ui.vertical(|ui| {
            ui.set_min_width(rect.width());

            let scroll_id = egui::Id::new("scroll_area").with(index);

            egui::ScrollArea::vertical().id_source(scroll_id).show(ui, |ui| {
                // Draw separating line
                let start_point = Pos2::new(rect.max.x, rect.min.y);
                let end_point = Pos2::new(rect.max.x, rect.max.y);
                let stroke = Stroke::new(2.0, _self.theme_mode.get_theme().0.stroke);
                painter.line_segment([start_point, end_point], stroke);

                let folders = get_folders(curr_path);

                // Draw each item of the page
                for path_obj in folders.unwrap() {
                    let item_name = path_obj.file_name().unwrap().to_str().unwrap();
                    let item_icon = if path_obj.is_dir() { "📁" } else { "📄" };


                    let mut item_button = Button::new(RichText::new(format!("{} {}", item_icon, item_name))
                        .size(16.0));

                    if !path_obj.is_dir() {
                        item_button = item_button.fill(Color32::TRANSPARENT);
                        item_button = item_button.stroke(Stroke::NONE);
                    } else if _self.pages.to_str().unwrap().contains(path_obj.to_str().unwrap()) {
                        item_button = item_button.fill(_self.theme_mode.get_theme().0.tertiary);
                    }

                    let button_response = ui.add(item_button);

                    if button_response.clicked() {
                        if path_obj.is_dir() {
                            let index_usize = (index.to_owned()+1) as usize;
                            let pages_count = _self.pages.iter().count();

                            // If a parent folder is selected
                            if index_usize < pages_count {
                                let mut components: Vec<_> = _self.pages.components().collect();

                                components.truncate(index_usize + 1);

                                // Rebuild the PathBuf from the modified components
                                let mut modified_path = PathBuf::new();

                                for component in components {
                                    modified_path.push(component.as_os_str());
                                }

                                // Replace the existing pages with the modified version
                                _self.pages = modified_path;

                            } else {
                                _self.add_page(item_name);
                            }
                        } else {
                            // Add metadata to the highlighted file
                            let metadata = fs::metadata(&path_obj).unwrap();

                            let file_data = FileData {
                                path: path_obj.to_str().unwrap().to_owned(),
                                name: item_name.to_owned(),
                                size: metadata.len(),
                                file_type: path_obj.extension().unwrap().to_str().unwrap().to_owned(),
                                creation_time: metadata.created().unwrap(),
                                last_access_time: metadata.accessed().unwrap(),
                                last_modification_time: metadata.modified().unwrap(),
                            };
                            _self.highlighted_file = Some(file_data.clone());
                        }
                    }

                    // Handles right-click on individual items
                    if button_response.clicked_by(PointerButton::Secondary) {
                        _self.folder_context_menu = item_name.to_owned();
                    }

                    if _self.folder_context_menu.eq(item_name) {
                        button_response.context_menu(|ui| {
                            if ui.button("Delete").clicked() {
                                if path_obj.is_file() {
                                    delete_file(&format!("{}\\{}", curr_path, item_name)).expect("File deletion expected");
                                }

                                _self.folder_context_menu = String::new();
                            }
                        });
                    }
                }
            });
        });
    });

    if response.clicked_by(PointerButton::Secondary) {
        _self.context_menu_open = true;
    }

    if _self.context_menu_open {
        response.context_menu(|ui| {
            if ui.button("Copy path").clicked() {
                println!("{}", curr_path);
                _self.context_menu_open = false;
                ui.output_mut(|o| o.copied_text = get_clean_abs_path(curr_path).expect("Expected clean abs path").to_str().unwrap().to_owned());
            }

            if ui.button("Create file").clicked() {
                create_file(curr_path, "test-text", "txt").expect("File creation expected");
                _self.context_menu_open = false;
            }
        });
    }
}

pub fn get_folders(root_path: &str) -> io::Result<Vec<PathBuf>> {
    let mut folder_vec = Vec::new();

    let read_dir = fs::read_dir(root_path);

    match read_dir {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        folder_vec.push(entry.path())
                    },
                    Err(e) => eprintln!("Error: {e}")
                }
            }

            Ok(folder_vec)
        }
        Err(e) => Err(e)
    }

}