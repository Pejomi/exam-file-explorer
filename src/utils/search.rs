use std::sync::{Arc, Mutex};
use std::time::Instant;
use walkdir::WalkDir;
use rayon::prelude::*;

pub fn search_for_files(root: String, target_file_name: &str, files: Arc<Mutex<Vec<String>>>) {
    let start_time = Instant::now();

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
            println!("{}", path);
            files.lock().unwrap().push(path);
        });

    let elapsed_time = start_time.elapsed(); // Measure elapsed time
    println!("Search completed in {} seconds", elapsed_time.as_secs_f32());
}
