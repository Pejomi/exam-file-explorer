use crate::app::App;
use crate::utils::files::*;

use egui::{Label, Sense};

pub fn build_navigation_bar(ui: &mut egui::Ui, app: &mut App) {
    if ui.button("⬅").clicked() {
        app.pages.pop();
    }
    // show current path and copy by click
    let current_path = get_clean_abs_path(app.pages.to_str().expect("Get clean abs path")).unwrap().to_str().unwrap().to_owned();

    egui::ScrollArea::horizontal().id_source("heading_scroll").show(ui, |ui| {
        if ui.add(Label::new("🏠 ".to_owned() + &current_path).sense(Sense::click())).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            ui.output_mut(|o| o.copied_text = String::from(&current_path));
        }
    });
}