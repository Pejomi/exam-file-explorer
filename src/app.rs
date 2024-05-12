use super::*;

#[derive(Clone)]
pub struct FolderContents {
    pub items: Vec<PathBuf>,
}

impl FolderContents {
    pub fn new(items: Vec<PathBuf>) -> Self {
        FolderContents { items }
    }
}

#[derive(Default)]
pub struct App {
    pub pages: Vec<FolderContents>,
    pub search_query: String,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
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

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ui::update_ui(ctx, self);
    }
}