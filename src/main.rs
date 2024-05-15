mod utils;
mod app;
mod structs;
mod ui;

use std::fs;
use eframe::{self, egui, Frame, NativeOptions};
use egui::{Button, RichText, Color32, Stroke, Sense, PointerButton, Pos2, Vec2, ViewportBuilder};

use crate::app::MyApp;

use utils::folders::*;
use utils::files::*;
use structs::file_data::FileData;


fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Option::from(Vec2::new(1000.0, 500.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Superior File Explorer - PeJoMi",
        options,
        Box::new(|_cc| Box::new(MyApp::new())));
}


fn ui_folders(ui: &mut egui::Ui, _self: &mut MyApp, index: &i32, curr_path: &str, screen_height: &f32) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(200.0, *screen_height - 100.0),
        Sense::click(),
    );

    let mut painter = ui.painter_at(rect).to_owned();

    painter.rect_filled(rect, 0.0, Color32::TRANSPARENT);

    ui.allocate_ui_at_rect(rect, |ui| {
        ui.vertical(|ui| {
            let scroll_id = egui::Id::new("scroll_area").with(index);

            egui::ScrollArea::vertical().id_source(scroll_id).show(ui, |ui| {
                // Draw separating line
                let start_point = Pos2::new(rect.max.x, rect.min.y);
                let end_point = Pos2::new(rect.max.x, rect.max.y);
                let stroke = Stroke::new(2.0, Color32::DARK_GRAY);
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
                        let selected_bg: Color32 = if ui.visuals().dark_mode {
                            Color32::from_rgb(59, 40, 204)
                        } else {
                            Color32::from_rgb(135, 191, 255)
                        };
                        item_button = item_button.fill(selected_bg);
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


// button.context_menu(|ui| {
// if ui.button("Copy path").clicked() {
// ui.output_mut(|o| o.copied_text = String::from(path_obj.as_path().to_str().unwrap()));
// ui.close_menu();
// }
//
// if ui.button("Open...").clicked() {
// ui.close_menu();
// }
//
// ui.menu_button("SubMenu", |ui| {
// ui.menu_button("SubMenu", |ui| {
// if ui.button("Open...").clicked() {
// ui.close_menu();
// }
// let _ = ui.button("Item");
// });
//
// ui.menu_button("SubMenu", |ui| {
// if ui.button("Open...").clicked() {
// ui.close_menu();
// }
// let _ = ui.button("Item");
// });
//
// let _ = ui.button("Item");
//
// if ui.button("Open...").clicked() {
// ui.close_menu();
// }
// });
//
// ui.menu_button("SubMenu", |ui| {
// let _ = ui.button("Item1");
// let _ = ui.button("Item2");
// let _ = ui.button("Item3");
// let _ = ui.button("Item4");
//
// if ui.button("Open...").clicked() {
// ui.close_menu();
// }
// });
//
// let _ = ui.button("Very long text for this item");
// });