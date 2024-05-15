use std::path::PathBuf;
use crate::app::MyApp;
use crate::ui_folders;
use egui::{Context, ScrollArea};

pub fn build_directory_list(ui: &mut egui::Ui,ctx: &Context, app: &mut MyApp) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        app.pages.to_str().unwrap();

        let self_clone = app.clone();
        let mut counter = 0;
        let screen_size = ctx.available_rect();
        let mut path = PathBuf::new();

        // Horizontal area for the pages
        ui.horizontal(|ui| {
            ui.set_max_width(&screen_size.width() - 220.0);
            ScrollArea::horizontal().id_source("body_scroll").show(ui, |ui| {
                for page in self_clone.pages.components() {
                    let curr_folder_name = page.as_os_str().to_str().unwrap();
                    path.push(curr_folder_name);

                    ui_folders(ui, app, &counter, path.to_str().unwrap(), &screen_size.height());
                    counter += 1;

                    // Spacer area between pages
                    ui.vertical(|ui| {
                        ui.set_width(30.0);
                    });
                }
            });
        });
    });
}