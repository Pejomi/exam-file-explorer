use egui::ScrollArea;
use egui_extras::{Size, StripBuilder};
use crate::app::App;
use crate::utils::files::{convert_byte_size, system_time_to_date_time};

pub fn build_info_panel(ui: &mut egui::Ui, app: &mut App) {
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
                                                ui.label("Created");
                                            });
                                        });
                                        strip.cell(|ui| {
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(file_data.file_type.to_string());
                                            });
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(convert_byte_size(file_data.size));
                                            });

                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(system_time_to_date_time(file_data.creation_time)); // TODO replace with date
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