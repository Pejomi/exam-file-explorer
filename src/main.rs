mod utils;
mod app;

use std::path::{Component, Components, PathBuf};
use std::{fs, vec};
use fs::canonicalize;
use eframe::{self, egui, Frame};
use egui::{Context, Button, RichText, Color32, Stroke, Label, Sense, PointerButton};
use std::vec::Vec;
use crate::app::MyApp;
use egui::Shape::Path;

fn main() {
    let options = eframe::NativeOptions::default();

    let _ = eframe::run_native(
        "Superior File Explorer - PeJoMi",
        options,
        Box::new(|_cc| Box::new(MyApp::new())));
}



fn ui_folders(ui: &mut egui::Ui, _self: &mut MyApp, index: &i32, curr_path: &str, screen_height: &f32) {
    ui.vertical(|ui| {

        ui.set_max_height(*screen_height);
        ui.set_width(200.0);

        let scroll_id = egui::Id::new("scroll_area").with(index);

        egui::ScrollArea::vertical().id_source(scroll_id).show(ui, |ui| {
            if ui.button("Add txt file test").clicked(){
                utils::create_file(curr_path, "text-test", "txt").expect("File creation expected");
            }

            for path_obj in utils::get_folders(curr_path) {
                let folder_name = path_obj.file_name().unwrap().to_str().unwrap();
                let item_icon = if path_obj.is_dir(){"📁"} else {"📄"};

                let mut item_button = Button::new(RichText::new(format!("{} {}", item_icon, folder_name))
                    .size(16.0));

                if !path_obj.is_dir(){
                    item_button = item_button.fill(Color32::TRANSPARENT);
                    item_button = item_button.stroke(Stroke::NONE);
                }

                /* let response = ui.add(item_button);
                if response.clicked(){
                    pages.push(FolderContents::new(utils::get_folders(path_obj.as_path().to_str().unwrap()))); */
                if ui.add(item_button).clicked(){
                    _self.add_page(folder_name);
                }
                // response.context_menu(|ui|
                //     {
                //         if ui.button("Copy path").clicked() {
                //             ui.output_mut(|o| o.copied_text = String::from(path_obj.as_path().to_str().unwrap()));
                //             ui.close_menu();
                //         }
                //         if ui.button("Open...").clicked() {
                //             ui.close_menu();
                //         }
                //         ui.menu_button("SubMenu", |ui| {
                //             ui.menu_button("SubMenu", |ui| {
                //                 if ui.button("Open...").clicked() {
                //                     ui.close_menu();
                //                 }
                //                 let _ = ui.button("Item");
                //             });
                //             ui.menu_button("SubMenu", |ui| {
                //                 if ui.button("Open...").clicked() {
                //                     ui.close_menu();
                //                 }
                //                 let _ = ui.button("Item");
                //             });
                //             let _ = ui.button("Item");
                //             if ui.button("Open...").clicked() {
                //                 ui.close_menu();
                //             }
                //         });
                //         ui.menu_button("SubMenu", |ui| {
                //             let _ = ui.button("Item1");
                //             let _ = ui.button("Item2");
                //             let _ = ui.button("Item3");
                //             let _ = ui.button("Item4");
                //             if ui.button("Open...").clicked() {
                //                 ui.close_menu();
                //             }
                //         });
                //         let _ = ui.button("Very long text for this item");
                //     }
                // );
            }
        });
    });
}
