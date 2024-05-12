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
    pub root_path: String,
}

impl App {
    pub fn new() -> Self {
        let root_path = String::from("C:\\");
        let mut app = App {
            pages: Vec::new(),
            search_query: String::new(),
            root_path: root_path.clone(),
        };
        app.initialize(&root_path);
        app
    }

    fn initialize(&mut self, root_path: &str) {
        self.pages = vec![FolderContents::new(utils::get_folders(root_path))];
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ui::update_ui(ctx, self);
    }
}