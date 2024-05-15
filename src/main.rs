mod utils;
mod app;
mod structs;
mod ui;

use eframe::{self, egui, NativeOptions};
use egui::{Vec2, ViewportBuilder};

use crate::app::App;

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
        Box::new(|_cc| Box::new(App::new())));
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