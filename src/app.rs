use eframe::Frame;
use egui::{Button, Color32, Context, Visuals};
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
                .size(Size::exact(20.0))// top
                // .size(Size::exact(20.0))// settings
                .size(Size::remainder().at_least(70.0)) // body
                .size(Size::exact(20.0)) // bottom
                .vertical(|mut strip| {
                    // top
                    strip.strip(|builder| {
                        builder.size(Size::remainder()).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                // ui.ctx().debug_painter().debug_rect(
                                //     ui.max_rect(),
                                //     Color32::YELLOW,
                                //     "Top",
                                // );

                                // ctx.set_pixels_per_point(1.5);
                                // ui.style_mut().spacing.button_padding = (12.0, 12.0).into();

                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {

                                    ui.button("⬅");
                                    ui.button("➡");
                                    ui.button("⬆");
                                    ui.button("⟳");
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
                        // ui.ctx().debug_painter().debug_rect(
                        //     ui.max_rect(),
                        //     Color32::BLUE,
                        //     "Body"
                        // );
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

                        egui::CentralPanel::default().show_inside(ui, |ui| {
                            // ui.vertical_centered(|ui| {
                            //     ui.heading("Central Panel");
                            // });

                            egui::ScrollArea::vertical().show(ui, |ui| {
                                // ui.set_max_width(ui.ctx().screen_rect().width());
                                    TableBuilder::new(ui)
                                        //.striped(true)
                                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                        .column(Column::remainder().resizable(true))
                                        .column(Column::remainder().resizable(true))
                                        .column(Column::remainder().resizable(true))
                                        .column(Column::remainder().resizable(true))
                                        .header(20.0, |mut header| {
                                            header.col(|ui| {
                                                ui.label("Name");
                                            });
                                            header.col(|ui| {
                                                ui.label("Date modified");
                                            });
                                            header.col(|ui| {
                                                ui.label("Type");
                                            });
                                            header.col(|ui| {
                                                ui.label("Size");
                                            });
                                        })
                                        .body(|mut body| {
                                            body.row(30.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.label("Hello");
                                                });
                                                row.col(|ui| {
                                                    ui.label("Hello");
                                                });
                                                row.col(|ui| {
                                                    ui.label("Hello");
                                                });
                                                row.col(|ui| {
                                                    ui.button("world!");
                                                });
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
                            .width_range(80.0..=500.0)
                            .show_inside(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.heading("Right Panel");
                                });
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.label("some text some text some text some text some text some text some text some text some text some text \nsome textsome textsome textsome textsome textsome textsome textsome textsome text\nsome textsome text");
                                });
                            });
                    });
                    // bottom
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                // ui.ctx().debug_painter().debug_rect(
                                //     ui.max_rect(),
                                //     Color32::GREEN,
                                //     "Bottom Left",
                                // );
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    ui.label("X items");
                                    ui.label("|");
                                    ui.label("X items selected");
                                    ui.label("|");
                                });
                            });
                            strip.cell(|ui| {
                                // ui.ctx().debug_painter().debug_rect(
                                //     ui.max_rect(),
                                //     Color32::GREEN,
                                //     "Bottom Right",
                                // );
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
            // ui.horizontal(|ui| {
            //     table
            // });
        });
    }
}
//todo: make a run appæ