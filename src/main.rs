mod utils;

use std::path::PathBuf;
use std::ptr::copy;
use std::vec;
use eframe::{self, egui, Frame};
use egui::{Context, Button, RichText, Color32, Stroke, Label, Sense};
use std::vec::Vec;


fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Superior File Explorer - PeJoMi",
        options,
        Box::new(|_cc| Box::new(MyApp::new())));
}

#[derive(Clone)]
struct FolderContents {
    items: Vec<PathBuf>
}

impl FolderContents {
    fn new(items: Vec<PathBuf>) -> Self {
        FolderContents { items }
    }
}

#[derive(Default)]
struct MyApp {
    pages: Vec<FolderContents>
}

impl MyApp {
    fn new() -> Self {
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
            ui.heading("C:\\");

            let pages_clone = self.pages.clone();
            let mut counter = 1;
            let screen_size = ctx.available_rect();

            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for page in pages_clone {
                        ui_folders(ui, &page.items, &mut self.pages, &counter, &screen_size.height());
                        counter += 1;
                    }
                });
            });
        });
    }
}

fn ui_folders(ui: &mut egui::Ui, folders: &Vec<PathBuf>, pages: &mut Vec<FolderContents>, index: &i32, screen_height: &f32) {
    ui.vertical(|ui| {

        ui.set_max_height(*screen_height);
        ui.set_width(200.0);

        let scroll_id = egui::Id::new("scroll_area").with(index);

        egui::ScrollArea::vertical().id_source(scroll_id).show(ui, |ui| {
            for path_obj in folders {
                let folder_name = path_obj.file_name().unwrap().to_str().unwrap();
                let item_icon = if path_obj.is_dir(){"📁"} else {"📄"};

                let mut item_button = Button::new(RichText::new(format!("{} {}", item_icon, folder_name))
                    .size(16.0));

                if !path_obj.is_dir(){
                    item_button = item_button.fill(Color32::TRANSPARENT);
                    item_button = item_button.stroke(Stroke::NONE);
                }

                let response = ui.add(item_button);
                if response.clicked(){
                    pages.push(FolderContents::new(utils::get_folders(path_obj.as_path().to_str().unwrap())));
                }
                response.context_menu(|ui|
                    {
                        if ui.button("Copy path").clicked() {
                            ui.output_mut(|o| o.copied_text = String::from(path_obj.as_path().to_str().unwrap()));
                            ui.close_menu();
                        }
                        if ui.button("Open...").clicked() {
                            ui.close_menu();
                        }
                        ui.menu_button("SubMenu", |ui| {
                            ui.menu_button("SubMenu", |ui| {
                                if ui.button("Open...").clicked() {
                                    ui.close_menu();
                                }
                                let _ = ui.button("Item");
                            });
                            ui.menu_button("SubMenu", |ui| {
                                if ui.button("Open...").clicked() {
                                    ui.close_menu();
                                }
                                let _ = ui.button("Item");
                            });
                            let _ = ui.button("Item");
                            if ui.button("Open...").clicked() {
                                ui.close_menu();
                            }
                        });
                        ui.menu_button("SubMenu", |ui| {
                            let _ = ui.button("Item1");
                            let _ = ui.button("Item2");
                            let _ = ui.button("Item3");
                            let _ = ui.button("Item4");
                            if ui.button("Open...").clicked() {
                                ui.close_menu();
                            }
                        });
                        let _ = ui.button("Very long text for this item");
                    }
                );
            }
        });
    });
}
