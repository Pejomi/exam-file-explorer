mod utils;
mod ui;
mod imports;
mod app;

use imports::*;
use app::App;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Superior File Explorer - PeJoMi",
        options,
        Box::new(|_cc| Box::new(App::new())));
}


