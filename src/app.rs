use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use eframe::Frame;

use egui::{Context, ScrollArea, Visuals};

use crate::{utils};
use egui_extras::{Size, StripBuilder};

use utils::folders::*;
use crate::structs::file_data::FileData;

use crate::ui::search_bar::build_search_bar;
use crate::ui::top_bar_navigation::build_navigation_bar;
use crate::ui::directory_list::build_directory_list;
use crate::ui::info_panel::build_info_panel;

#[derive(Default, Clone)]
pub(crate) struct MyApp {
    pub(crate) pages: PathBuf,
    pub(crate) start_dir: String,
    pub(crate) files: Arc<Mutex<Vec<String>>>,
    pub(crate) searching: Arc<Mutex<bool>>,
    pub(crate) search_query: String,
    pub(crate) search_result_menu_open: bool,
    pub(crate) highlighted_file: Option<FileData>,
    pub(crate) context_menu_open: bool,
}

impl MyApp {
    pub(crate) fn new() -> Self {
        let mut app = MyApp {
            pages: PathBuf::new(),
            start_dir: String::from("test-directory"),
            files: Arc::new(Mutex::new(Vec::new())),
            searching: Arc::new(Mutex::new(false)),
            search_query: String::new(),
            search_result_menu_open: false,
            highlighted_file: None,
            context_menu_open: false,
        };
        app.initialize();
        app
    }

    fn initialize(&mut self) {
        self.pages = PathBuf::from(&self.start_dir);
    }
    pub(crate) fn add_page(&mut self, folder_name: &str) {
        self.pages.push(folder_name);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::exact(20.0))// top
                .size(Size::remainder().at_least(70.0)) // body
                .size(Size::exact(20.0)) // bottom
                .vertical(|mut strip| {
                    // Top panel
                    strip.strip(|builder| {
                        builder.size(Size::remainder())
                            .size(Size::relative(0.25).at_least(200.0))
                            .horizontal(|mut strip| {
                                // Navigation
                                strip.cell(|ui| {
                                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        build_navigation_bar(ui, self);
                                    });
                                });
                                // Search bar
                                strip.cell(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        build_search_bar(ui, self);
                                    });
                                });
                            });
                    });
                    // Body
                    strip.cell(|ui| {
                        ui.separator();
                        egui::CentralPanel::default().show_inside(ui, |ui| {
                            ScrollArea::vertical().show(ui, |ui| {
                                // Directory list
                                build_directory_list(ui, ctx, self);
                                // Right panel
                                build_info_panel(ui, self);
                            });
                        });
                    });
                    // Bottom
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            // items count description
                            strip.cell(|ui| {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    let items_amount = get_folders(self.pages.as_path().to_str().unwrap()).iter().count();
                                    ui.label(format!("{} items", items_amount)); // todo: replace X
                                    ui.label("|");
                                });
                            });

                            // light/dark mode settings
                            strip.cell(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("☀/🌙").clicked() {
                                        let visuals = if ui.visuals().dark_mode {
                                            Visuals::light()
                                        } else {
                                            Visuals::dark()
                                        };
                                        ctx.set_visuals(visuals);
                                    }
                                });
                            });
                        });
                    });
                });
        });
    }
}