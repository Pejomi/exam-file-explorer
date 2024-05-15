use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileData {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub file_type: String,
    pub creation_time: SystemTime,
    pub last_access_time: SystemTime,
    pub last_modification_time: SystemTime,
}