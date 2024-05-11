use super::*;

pub fn update_ui(ctx: &Context, app: &mut MyApp) {
    egui::TopBottomPanel::top("top_panel")
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.add(egui::TextEdit::singleline(&mut app.search_query).desired_width(100.0));
            });
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("C:\\");

        let pages_clone = app.pages.clone();
        let mut counter = 1;
        let screen_size = ctx.available_rect();

        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                for page in pages_clone {
                    ui_folders(ui, &page.items, &mut app.pages, &counter, &screen_size.height());
                    counter += 1;
                }
            });
        });
    });
}

fn ui_folders(ui: &mut egui::Ui, folders: &Vec<PathBuf>, pages: &mut Vec<FolderContents>, index: &i32, screen_height: &f32) {
    ui.vertical(|ui| {
        ui.set_max_height(*screen_height);
        ui.set_width(200.0);

        let scroll_id = egui::Id::new("scroll_area").with(index);

        egui::ScrollArea::vertical().id_source(scroll_id).show(ui, |ui| {
            for path_obj in folders {
                let folder_name = path_obj.file_name().unwrap().to_str().unwrap();
                let item_icon = if path_obj.is_dir() { "📁" } else { "📄" };

                let mut item_button = Button::new(RichText::new(format!("{} {}", item_icon, folder_name))
                    .size(16.0));

                if !path_obj.is_dir() {
                    item_button = item_button.fill(Color32::TRANSPARENT);
                    item_button = item_button.stroke(Stroke::NONE);
                }

                if ui.add(item_button).clicked() {
                    pages.push(FolderContents::new(utils::get_folders(path_obj.as_path().to_str().unwrap())));
                }
            }
        });
    });
}
