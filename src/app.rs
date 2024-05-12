use eframe::Frame;
use egui::{Color32, Context};
use crate::{FolderContents, ui_folders, utils};
use egui_extras::{TableBuilder, Column};
use egui_extras::{Size, StripBuilder};
#[derive(Default)]
pub(crate) struct MyApp {
    pages: Vec<FolderContents>
}

impl MyApp {
    pub(crate) fn new() -> Self {
        let mut app = MyApp {
            pages: Vec::new()
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
            // let table = TableBuilder::new(ui)
            //     .striped(true)
            //     .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            //     .column(Column::auto().resizable(true))
            //     .column(Column::auto())
            //     .header(20.0, |mut header| {
            //         header.col(|ui| {
            //             ui.heading("First column");
            //         });
            //         header.col(|ui| {
            //             ui.heading("Second column");
            //         });
            //     })
            //     .body(|mut body| {
            //         body.row(30.0, |mut row| {
            //             row.col(|ui| {
            //                 ui.label("Hello");
            //             });
            //             row.col(|ui| {
            //                 ui.button("world!");
            //             });
            //         });
            //     });

            StripBuilder::new(ui)
                .size(Size::exact(40.0))// top
                .size(Size::exact(40.0))// settings
                .size(Size::remainder().at_least(70.0)) // body
                .size(Size::exact(20.0)) // bottom
                .vertical(|mut strip| {
                    // top
                    strip.strip(|builder| {
                        builder.size(Size::remainder()).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.ctx().debug_painter().debug_rect(
                                    ui.max_rect(),
                                    Color32::YELLOW,
                                    "Top",
                                );
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    ui.label("Top Left");
                                    ui.label("Top Right");
                                });
                            });
                        });
                    });
                    // settings
                    strip.strip(|builder| {
                        builder.size(Size::remainder()).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.ctx().debug_painter().debug_rect(
                                    ui.max_rect(),
                                    Color32::RED,
                                    "Settings",
                                );
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    ui.label("Top Left");
                                    ui.label("Top Right");
                                });
                            });
                        });
                    });
                    // body
                    strip.cell(|ui| {
                        ui.ctx().debug_painter().debug_rect(
                            ui.max_rect(),
                            Color32::BLUE,
                            "Body"
                        );
                        egui::SidePanel::left("left_panel")
                            .resizable(true)
                            .default_width(200.0)
                            .width_range(80.0..=500.0)
                            .show_inside(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.heading("Left Panel");
                                });
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.label("some text some text some text some text some text some text some text some text some text some text \nsome textsome textsome textsome textsome textsome textsome textsome textsome text\nsome textsome text");
                                });
                            });
                        egui::SidePanel::right("right_panel")
                            .resizable(true)
                            .default_width(200.0)
                            .width_range(80.0..=500.0)
                            .show_inside(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.heading("Right Panel");
                                });
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.label("some text some text some text some text some text some text some text some text some text some text \nsome textsome textsome textsome textsome textsome textsome textsome textsome text\nsome textsome text");
                                });
                            });
                        egui::CentralPanel::default().show_inside(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.heading("Central Panel");
                            });
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.label("some text some text some text some text some text some text some text some text some text some text \nsome textsome textsome textsome textsome textsome textsome textsome textsome text\nsome textsome text");
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
                    });
                    // bottom
                    strip.strip(|builder| {
                        builder.size(Size::remainder()).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.ctx().debug_painter().debug_rect(
                                    ui.max_rect(),
                                    Color32::GREEN,
                                    "Bottom",
                                );
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    ui.label("X items");
                                    ui.label("|");
                                    ui.label("X items selected");
                                    ui.label("|");
                                });
                            });
                        });
                    });
                });
            // ui.horizontal(|ui| {
            //     table
            // });
        });
    }
}
//todo: make a run appæ