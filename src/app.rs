use std::path::PathBuf;
use eframe::Frame;
use egui::{Button, CollapsingHeader, Color32, Context, RichText, TextEdit, Visuals, Pos2, AboveOrBelow};
use egui::AboveOrBelow::Above;
use crate::{ui_folders, utils};
use egui_extras::{TableBuilder, Column};
use egui_extras::{Size, StripBuilder};
#[derive(Default, Clone)]
pub(crate) struct MyApp {
    pages: PathBuf,
    start_dir: String,
    search_query: String,
    pub(crate) context_menu_open: bool
}

impl MyApp {
    pub(crate) fn new() -> Self {
        let mut app = MyApp {
            pages: PathBuf::new(),
            start_dir: String::from("test-directory"),
            search_query: String::new(),
            context_menu_open: false
        };
        app.initialize();
        app
    }

    fn initialize(&mut self) {
        self.pages = PathBuf::from(&self.start_dir);
    }
    pub(crate) fn add_page(&mut self, folder_name: &str){
        self.pages.push(folder_name);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::exact(20.0))// top
                // .size(Size::exact(20.0))// settings
                .size(Size::remainder().at_least(70.0)) // body
                .size(Size::exact(20.0)) // bottom
                .vertical(|mut strip| {
                    // top
                    strip.strip(|builder| {
                        builder.size(Size::remainder()).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    ui.button("⬅");
                                    ui.button("➡");
                                    ui.button("⬆");
                                    ui.button("⟳");
                                    // todo: show current path (input field)
                                    // todo: add search (input field)
                                    ui.add(egui::TextEdit::singleline(&mut self.search_query).desired_width(100.0));
                                    if ui.button("Search").clicked() {
                                        // Trigger search action here
                                        //search_by_name(&app.root_path, &app.search_query);
                                        println!("Search query: {}", self.search_query)
                                    }
                                });
                            });
                        });
                    });

                    // body
                    strip.cell(|ui| {
                        egui::CentralPanel::default().show_inside(ui, |ui| {
                            let rel_path = self.pages.to_str().unwrap();

                            // Horizontal area for the heading
                            ui.horizontal(|ui| {
                                ui.set_max_width(700.0);

                                egui::ScrollArea::horizontal().id_source("heading_scroll").show(ui, |ui| {
                                    ui.heading(RichText::new(utils::get_clean_abs_path(rel_path).to_str().unwrap()).size(13.0));
                                });
                            });

                            let self_clone = self.clone();
                            let mut counter = 0;
                            let screen_size = ctx.available_rect();
                            let mut path = PathBuf::new();

                            // Horizontal area for the pages
                            ui.horizontal(|ui| {
                                ui.set_max_width(&screen_size.width() - 220.0);
                                egui::ScrollArea::horizontal().id_source("body_scroll").show(ui, |ui| {

                                    for page in self_clone.pages.components() {
                                        let curr_folder_name = page.as_os_str().to_str().unwrap();
                                        path.push(curr_folder_name);

                                        ui_folders(ui, self, &counter, path.to_str().unwrap(), &screen_size.height());
                                        counter += 1;

                                        // Spacer area between pages
                                        ui.vertical(|ui| {
                                            ui.set_width(30.0);
                                        });
                                    }
                                });
                            });
                        });

                        egui::SidePanel::right("right_panel")
                            .resizable(true)
                            .default_width(200.0)
                            .width_range(200.0..=500.0)
                            .show_inside(ui, |ui| {
                                ui.vertical(|ui| {
                                    egui::ScrollArea::vertical().show(ui, |ui| {
                                        // description of a clicked file
                                        ui.heading("File name"); //todo: insert clicked file name here
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
                                                                ui.label("Some text") // todo: replace dummy text
                                                            });
                                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                                ui.label("Some text") // todo: replace dummy text
                                                            });
                                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                                ui.label("Some text") // todo: replace dummy text
                                                            });
                                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                                ui.label("Some text") // todo: replace dummy text
                                                            });
                                                        });
                                                    });
                                                });
                                            });
                                    });
                                });
                            });
                    });

                    // bottom
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            // items and selected items description
                            strip.cell(|ui| {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    ui.label("X items"); // todo: replace X
                                    ui.label("|");
                                    ui.label("X items selected"); // todo: replace X
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
//todo: make a run app method