use eframe::Frame;
use egui::{Button, CollapsingHeader, Color32, Context, TextEdit, Visuals};
use crate::{FolderContents, ui_folders, utils};
use egui_extras::{TableBuilder, Column};
use egui_extras::{Size, StripBuilder};
#[derive(Default)]
pub(crate) struct MyApp {
    pages: Vec<FolderContents>,
    search_query: String
}

impl MyApp {
    pub(crate) fn new() -> Self {
        let mut app = MyApp {
            pages: Vec::new(),
            search_query: String::new()
        };
        app.initialize();
        app
    }

    fn initialize(&mut self) {
        self.pages = vec![FolderContents::new(utils::get_folders("C:\\"))];
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
                                let pages_clone = self.pages.clone();
                                let mut counter = 1;
                                let screen_size = ui.max_rect();
                                egui::ScrollArea::horizontal().show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        for page in pages_clone {
                                            ui_folders(ui, &page.items, &mut self.pages, &counter, &screen_size.height());
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