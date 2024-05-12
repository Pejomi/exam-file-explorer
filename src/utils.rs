use super::*;

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

