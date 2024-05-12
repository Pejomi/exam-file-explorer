mod utils;
mod ui;
mod imports;

use imports::*;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Superior File Explorer - PeJoMi",
        options,
        Box::new(|_cc| Box::new(MyApp::new())));
}

#[derive(Clone)]
struct FolderContents {
    items: Vec<PathBuf>,
}

impl FolderContents {
    fn new(items: Vec<PathBuf>) -> Self {
        FolderContents { items }
    }
}

#[derive(Default)]
struct MyApp {
    pages: Vec<FolderContents>,
    search_query: String,
}

impl MyApp {
    fn new() -> Self {
        let mut app = MyApp {
            pages: Vec::new(),
            search_query: String::new(),
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
        ui::update_ui(ctx, self);
    }
}
