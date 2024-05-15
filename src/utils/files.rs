use std::fs::{canonicalize, File};
use std::io;
use std::path::PathBuf;

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