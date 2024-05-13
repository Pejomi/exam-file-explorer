use std::fs;
use std::path::PathBuf;
use egui::Color32;
use std::fs::File;
use std::io;
use fs::canonicalize;

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

pub fn create_file(file_path: &str, file_name: &str, file_type: &str) -> io::Result<String> {
    let complete_path = format!("{}\\{}.{}", file_path, file_name, file_type);
    File::create(&complete_path)?;
    Ok(complete_path)
}

pub fn get_clean_abs_path(rel_path: &str) -> PathBuf {
    let raw_path = canonicalize(rel_path).unwrap();

    if raw_path.to_str().unwrap().starts_with(r"\\?\") {
        let path = raw_path.display().to_string();
        PathBuf::from(&path[4..])
    } else {
        raw_path
    }
    
}

