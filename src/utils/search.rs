use walkdir::WalkDir;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn search_for_files(root: &str, target_file_name: &str, files: Arc<Mutex<Vec<String>>>) {
    WalkDir::new(root)
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            entry.path().is_file()
                && entry.file_name()
                .to_str()
                .map(|s| s.starts_with(target_file_name))
                .unwrap_or(false)
        })
        .for_each(|entry| {
            let path = entry.path().display().to_string();
            files.lock().unwrap().push(path);
        });
}
