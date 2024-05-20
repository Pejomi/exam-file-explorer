use std::path::PathBuf;
use std::fs;
use egui::{Button, RichText, Color32, Stroke, Sense, PointerButton, Pos2};

use utils::files::*;

use crate::app::App;
use crate::structs::file_data::FileData;
use crate::ui::theme::get_theme;
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
            let scroll_id = egui::Id::new("scroll_area").with(index);

            egui::ScrollArea::vertical().id_source(scroll_id).show(ui, |ui| {
                // Draw separating line
                let start_point = Pos2::new(rect.max.x, rect.min.y);
                let end_point = Pos2::new(rect.max.x, rect.max.y);
                let stroke = Stroke::new(2.0, get_theme(_self.theme_mode.clone()).0.stroke);
                painter.line_segment([start_point, end_point], stroke);

                // Draw each item of the page
                for path_obj in get_folders(curr_path) {
                    let folder_name = path_obj.file_name().unwrap().to_str().unwrap();
                    let item_icon = if path_obj.is_dir() { "📁" } else { "📄" };


                    let mut item_button = Button::new(RichText::new(format!("{} {}", item_icon, folder_name))
                        .size(16.0));

                    if !path_obj.is_dir() {
                        item_button = item_button.fill(Color32::TRANSPARENT);
                        item_button = item_button.stroke(Stroke::NONE);
                    } else if _self.pages.to_str().unwrap().contains(path_obj.to_str().unwrap()) {
                        item_button = item_button.fill(get_theme(_self.theme_mode.clone()).0.tertiary);
                    }

                    if ui.add(item_button).clicked() {
                        if path_obj.is_dir() {
                            _self.add_page(folder_name);
                        } else {
                            let metadata = fs::metadata(&path_obj).unwrap();

                            let file_data = FileData {
                                path: path_obj.to_str().unwrap().to_owned(),
                                name: folder_name.to_owned(),
                                size: metadata.len(),
                                file_type: "txt".to_owned(),
                                creation_time: metadata.created().unwrap(),
                                last_access_time: metadata.accessed().unwrap(),
                                last_modification_time: metadata.modified().unwrap(),
                            };
                            _self.highlighted_file = Some(file_data.clone());
                        }
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
                ui.output_mut(|o| o.copied_text = get_clean_abs_path(curr_path).to_str().unwrap().to_owned());
            }

            if ui.button("Create file").clicked() {
                create_file(curr_path, "test-text", "txt").expect("File creation expected");
                _self.context_menu_open = false;
            }
        });
    }
}

pub fn get_folders(root_path: &str) -> Vec<PathBuf> {
    let mut folder_vec = Vec::new();

    match fs::read_dir(root_path){
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        folder_vec.push(entry.path())
                    },
                    Err(e) => eprintln!("Error: {e}")
                }
            }
        }
        Err(e) => eprintln!("Error: {e}")
    }

    folder_vec
}