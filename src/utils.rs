use std::fs;
use std::path::PathBuf;
use egui::Color32;

pub fn get_folders(root_path: &str) -> Vec<PathBuf> {
    let mut folder_vec = Vec::new();

    match fs::read_dir(root_path){
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        folder_vec.push(entry.path())
                    },
                    Err(e) => eprintln!("Error: {e}")
                }
            }
        }
        Err(e) => eprintln!("Error: {e}")
    }

    folder_vec
}

pub fn hex_to_color32(hex: &str) -> Color32 {
    let hex = hex.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);

    Color32::from_rgb(r, g, b)
}
