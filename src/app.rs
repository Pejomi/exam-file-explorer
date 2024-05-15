use std::path::PathBuf;
use eframe::Frame;
use egui::{Context, Label, Sense, Visuals};

use crate::{ui_folders, utils};
use egui_extras::{Size, StripBuilder};
#[derive(Default, Clone)]
pub(crate) struct MyApp {
    pub(crate) pages: PathBuf,
    start_dir: String,
    search_query: String
}

impl MyApp {
    pub(crate) fn new() -> Self {
        let mut app = MyApp {
            pages: PathBuf::new(),
            start_dir: String::from("test-directory"),
            search_query: String::new()
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
                        builder.size(Size::remainder())
                            .size(Size::relative(0.25).at_least(200.0))
                            .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    if ui.button("⬅").clicked() {
                                        self.pages.pop();
                                    }
                                    // show current path and copy by click
                                    let binding = utils::get_clean_abs_path(self.pages.to_str().unwrap());
                                    let current_path = binding.to_str().unwrap();

                                    if ui.add(Label::new(String::from("🏠 ".to_owned() + current_path)).sense(Sense::click())).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                        ui.output_mut(|o| o.copied_text = String::from(current_path));
                                    }
                                });
                            });
                            strip.cell(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("Search").clicked() {
                                        // Trigger search action here
                                        //search_by_name(&app.root_path, &app.search_query);
                                        println!("Search query: {}", self.search_query)
                                    }
                                    ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut self.search_query));
                                });
                            });
                        });
                    });
                    // settings
                    // strip.strip(|builder| {
                    //     builder.size(Size::remainder()).horizontal(|mut strip| {
                    //         strip.cell(|ui| {
                    //             ui.ctx().debug_painter().debug_rect(
                    //                 ui.max_rect(),
                    //                 Color32::RED,
                    //                 "Settings",
                    //             );
                    //             ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    //                 ui.button("🗑");
                    //
                    //             });
                    //         });
                    //     });
                    // });


                    // body
                    strip.cell(|ui| {
                        // egui::SidePanel::left("left_panel")
                        //     .resizable(true)
                        //     .default_width(100.0)
                        //     .width_range(100.0..=500.0)
                        //     .show_inside(ui, |ui| {
                        //         ui.vertical(|ui| {
                        //         egui::ScrollArea::vertical().show(ui, |ui| {
                        //             // directory tree // todo: replace dummy text
                        //             CollapsingHeader::new("Home")
                        //                 .id_source("1")
                        //                 .default_open(false)
                        //                 .show(ui, |ui| {
                        //                     ui.label("hello hello hello hello hello hello");
                        //                     ui.label("hello");
                        //                     CollapsingHeader::new("Home")
                        //                         .default_open(false)
                        //                         .show(ui, |ui| {
                        //                             ui.label("hello");
                        //                             ui.label("hello");
                        //                             ui.label("hello")
                        //                         })
                        //                         .body_returned;
                        //                 })
                        //                 .body_returned;
                        //             });
                        //         });
                        //     });
                        ui.separator();
                        egui::CentralPanel::default().show_inside(ui, |ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
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
                                //directory list
                                // ui.heading(RichText::new(utils::get_clean_abs_path(self.pages.to_str().unwrap()).to_str().unwrap()).size(13.0));

                                let self_clone = self.clone();
                                let mut counter = 0;
                                let screen_size = ctx.available_rect();
                                let mut path = PathBuf::new();

                                egui::ScrollArea::horizontal().show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        for page in self_clone.pages.components() {
                                            let curr_folder_name = page.as_os_str().to_str().unwrap();
                                            path.push(curr_folder_name);

                                            ui_folders(ui, self, &counter, path.to_str().unwrap(), &screen_size.height());
                                            counter += 1;
                                        }
                                    });
                                });
                            });
                        });
                        //directory list
                        // let pages_clone = self.pages.clone();
                        // let mut counter = 1;
                        // let screen_size = ui.max_rect();
                        // egui::ScrollArea::horizontal().show(ui, |ui| {
                        //     ui.horizontal(|ui| {
                        //         for page in pages_clone {
                        //             ui_folders(ui, &page.items, &mut self.pages, &counter, &screen_size.height());
                        //             counter += 1;
                        //         }
                        //     });
                        // });
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
                            // items count description
                            strip.cell(|ui| {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    let items_amount = utils::get_folders(self.pages.as_path().to_str().unwrap()).iter().count();
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