use std::thread;
use crate::utils;

use crate::app::MyApp;

pub fn build_search_bar(ui: &mut egui::Ui, app: &mut MyApp) {
    if ui.button("Search").clicked() {
        app.search_result_menu_open = true;
        if !app.search_query.is_empty() {
            let files = app.files.clone();
            let searching = app.searching.clone();
            let query = app.search_query.clone();
            let path = app.start_dir.clone();

            *searching.lock().unwrap() = true;

            thread::spawn(move || {
                utils::search::search_for_files(path, &query, files);
                *searching.lock().unwrap() = false;
            });
        }
    }
    ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut app.search_query).hint_text("Enter file name"));
}