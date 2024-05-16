use std::path::PathBuf;
use std::{fs, io};

pub fn get_folders(root_path: &str) -> io::Result<Vec<PathBuf>> {
    let mut folder_vec = Vec::new();

    let read_dir = fs::read_dir(root_path);

    match read_dir {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        folder_vec.push(entry.path())
                    },
                    Err(e) => eprintln!("Error: {e}")
                }
            }

            Ok(folder_vec)
        }
        Err(e) => Err(e)
    }

}