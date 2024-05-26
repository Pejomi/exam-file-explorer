use std::fs::{canonicalize, File, remove_file};
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;
use chrono::{DateTime, Local};

pub fn create_file(file_path: &str, file_name: &str, file_type: &str) -> io::Result<String> {
    let mut complete_path = PathBuf::from(file_path);
    complete_path.push(format!("{}.{}", file_name, file_type));

    let file = File::create(&complete_path);

    match file {
        Ok(_) => Ok(complete_path.to_string_lossy().into_owned()),
        Err(e) => Err(e)
    }
}

pub fn delete_file(abs_file_path: &str) -> io::Result<String> {
    let file = remove_file(&abs_file_path);

    match file {
        Ok(_) => Ok(abs_file_path.to_owned()),
        Err(e) => Err(e)
    }
}

pub fn get_clean_abs_path(rel_path: &str) -> io::Result<PathBuf> {
    let raw_path = canonicalize(rel_path)?;

    let path_str = raw_path.to_str().ok_or_else(|| io::Error::new(
        io::ErrorKind::InvalidData,
        "Path contains invalid UTF-8 characters"
    ))?;

    if path_str.starts_with(r"\\?\") {
        Ok(PathBuf::from(&path_str[4..]))
    } else {
        Ok(raw_path)
    }

}

pub fn convert_byte_size(bytes: u64) -> String {
    const KILOBYTE: u64 = 1024;
    const MEGABYTE: u64 = KILOBYTE * 1024;
    const GIGABYTE: u64 = MEGABYTE * 1024;

    if bytes >= GIGABYTE {
        format!("{:.2} GB", bytes as f64 / GIGABYTE as f64)
    } else if bytes >= MEGABYTE {
        format!("{:.2} MB", bytes as f64 / MEGABYTE as f64)
    } else if bytes >= KILOBYTE {
        format!("{:.2} KB", bytes as f64 / KILOBYTE as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

pub fn system_time_to_date_time(system_time: SystemTime) -> String {
    let date_time: DateTime<Local> = DateTime::<Local>::from(system_time);
    date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}