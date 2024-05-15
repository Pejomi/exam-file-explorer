use crate::app::MyApp;
use crate::utils::files::*;

use egui::{Label, Sense};

pub fn build_navigation_bar(ui: &mut egui::Ui, app: &mut MyApp) {
    if ui.button("⬅").clicked() {
        app.pages.pop();
    }
// show current path and copy by click
    let binding = get_clean_abs_path(app.pages.to_str().unwrap());
    let current_path = binding.to_str().unwrap();

    egui::ScrollArea::horizontal().id_source("heading_scroll").show(ui, |ui| {
        if ui.add(Label::new(String::from("🏠 ".to_owned() + current_path)).sense(Sense::click())).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            ui.output_mut(|o| o.copied_text = String::from(current_path));
        }
    });
}