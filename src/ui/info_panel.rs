use egui::ScrollArea;
use egui_extras::{Size, StripBuilder};
use crate::app::MyApp;

pub fn build_info_panel(ui: &mut egui::Ui, app: &mut MyApp) {
    if let Some(file_data) = &app.highlighted_file {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(200.0..=500.0)
            .show_inside(ui, |ui| {
                ui.vertical(|ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        // description of a clicked file
                        ui.heading(file_data.name.as_str());
                        StripBuilder::new(ui)
                            .size(Size::exact(20.0))
                            .vertical(|mut strip| {
                                strip.strip(|builder| {
                                    builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                                        strip.cell(|ui| {
                                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                                ui.label("Type");
                                            });
                                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                                ui.label("Size");
                                            });
                                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                                ui.label("File location");
                                            });
                                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                                ui.label("Date modified");
                                            });
                                        });
                                        strip.cell(|ui| {
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(file_data.file_type.as_str())
                                            });
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(file_data.size.to_string());
                                            });
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(file_data.path.as_str())
                                            });
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label("TODO") // TODO replace with date
                                            });
                                        });
                                    });
                                });
                            });
                    });
                });
            });
    }

}