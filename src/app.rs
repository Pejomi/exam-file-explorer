use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use eframe::Frame;

use egui::{Context, Label, PointerButton, ScrollArea, Sense, Visuals};

use crate::{ui_folders, utils};
use egui_extras::{Size, StripBuilder};

use utils::files::*;
use utils::folders::*;

#[derive(Default, Clone)]
pub(crate) struct MyApp {
    pub(crate) pages: PathBuf,
    start_dir: String,
    files: Arc<Mutex<Vec<String>>>,
    searching: Arc<Mutex<bool>>,
    search_query: String,
    search_result_menu_open: bool,
    pub(crate) context_menu_open: bool
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
                .size(Size::remainder().at_least(70.0)) // body
                .size(Size::exact(20.0)) // bottom
                .vertical(|mut strip| {
                    // top
                    strip.strip(|builder| {
                        builder.size(Size::remainder())
                            .size(Size::relative(0.25).at_least(200.0))
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        if ui.button("⬅").clicked() {
                                            self.pages.pop();
                                        }
                                        // show current path and copy by click
                                        let binding = get_clean_abs_path(self.pages.to_str().unwrap());
                                        let current_path = binding.to_str().unwrap();

                                        egui::ScrollArea::horizontal().id_source("heading_scroll").show(ui, |ui| {
                                            if ui.add(Label::new(String::from("🏠 ".to_owned() + current_path)).sense(Sense::click())).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                ui.output_mut(|o| o.copied_text = String::from(current_path));
                                            }
                                        });
                                    });
                                });
                                strip.cell(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.button("Search").clicked() {
                                            self.search_result_menu_open = true;
                                            if !self.search_query.is_empty() {
                                                let files = self.files.clone();
                                                let searching = self.searching.clone();
                                                let query = self.search_query.clone();
                                                let path = self.start_dir.clone();

                                                *searching.lock().unwrap() = true;

                                                thread::spawn(move || {
                                                    utils::search::search_for_files(path, &query, files);
                                                    *searching.lock().unwrap() = false;
                                                });
                                            }
                                        }


                                        ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut self.search_query).hint_text("Enter file name"));
                                    });
                                });
                            });
                    });



                    // body
                    strip.cell(|ui| {
                        ui.separator();
                        egui::CentralPanel::default().show_inside(ui, |ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                //directory list
                                egui::CentralPanel::default().show_inside(ui, |ui| {
                                    let rel_path = self.pages.to_str().unwrap();

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
                        });
                    });
                    // bottom
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
//todo: make a run app method
// table display list
// TableBuilder::new(ui)
//     //.striped(true)
//     .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
//     .column(Column::remainder().resizable(true))
//     .column(Column::remainder().resizable(true))
//     .column(Column::remainder().resizable(true))
//     .column(Column::remainder().resizable(true))
//     .header(20.0, |mut header| {
//         header.col(|ui| {
//             ui.label("Name");
//         });
//         header.col(|ui| {
//             ui.label("Date modified");
//         });
//         header.col(|ui| {
//             ui.label("Type");
//         });
//         header.col(|ui| {
//             ui.label("Size");
//         });
//     })
//     .body(|mut body| { // todo: loop and display files from current path
//         body.row(30.0, |mut row| {
//             row.col(|ui| {
//                 ui.label("Hello");
//             });
//             row.col(|ui| {
//                 ui.label("Hello");
//             });
//             row.col(|ui| {
//                 ui.label("Hello");
//             });
//             row.col(|ui| {
//                 ui.button("world!");
//             });
//         });
//     });