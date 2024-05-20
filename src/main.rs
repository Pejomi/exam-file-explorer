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
        "Superior File Explorer 🦀 PeJoMi",
        options,
        Box::new(|_cc| Box::new(App::new())));
}


