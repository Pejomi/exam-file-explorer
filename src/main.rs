mod utils;

use std::path::{Component, Components, PathBuf};
use std::{fs, vec};
use fs::canonicalize;
use eframe::{self, egui, Frame};
use egui::{Context, Button, RichText, Color32, Stroke, PointerButton};
use std::vec::Vec;
use egui::Shape::Path;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Superior File Explorer - PeJoMi",
        options,
        Box::new(|_cc| Box::new(MyApp::new())));
}

#[derive(Default, Clone)]
struct MyApp {
    pages: PathBuf,
    start_dir: String
}

impl MyApp {
    fn new() -> Self {
        let mut app = MyApp {
            pages: PathBuf::new(),
            start_dir: String::from("test-directory")
        };
        app.initialize();
        app
    }

    fn initialize(&mut self) {
        self.pages = PathBuf::from(&self.start_dir);
    }

    fn add_page(&mut self, folder_name: &str){
        self.pages.push(folder_name);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading(RichText::new(utils::get_clean_abs_path(self.pages.to_str().unwrap()).to_str().unwrap()).size(13.0));

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
    }
}

fn ui_folders(ui: &mut egui::Ui, _self: &mut MyApp, index: &i32, curr_path: &str, screen_height: &f32) {
    ui.vertical(|ui| {

        ui.set_max_height(*screen_height);
        ui.set_width(200.0);

        let scroll_id = egui::Id::new("scroll_area").with(index);

        egui::ScrollArea::vertical().id_source(scroll_id).show(ui, |ui| {
            if ui.button("Add txt file test").clicked(){
                utils::create_file(curr_path, "text-test", "txt").expect("File creation expected");
            }

            for path_obj in utils::get_folders(curr_path) {
                let folder_name = path_obj.file_name().unwrap().to_str().unwrap();
                let item_icon = if path_obj.is_dir(){"📁"} else {"📄"};

                let mut item_button = Button::new(RichText::new(format!("{} {}", item_icon, folder_name))
                    .size(16.0));

                if !path_obj.is_dir(){
                    item_button = item_button.fill(Color32::TRANSPARENT);
                    item_button = item_button.stroke(Stroke::NONE);
                }

                if ui.add(item_button).clicked(){
                    _self.add_page(folder_name);
                }
            }
        });
    });
}
