use std::fs::create_dir;
use std::io;

pub fn use_dir(p: Option<String>) -> Result<String, io::Error> {
    let default_folder_name = String::from("todos");
    let path = p.unwrap_or(default_folder_name);
    match create_dir(&path) {
        Ok(_) => Ok(path),
        Err(e) => {
            if e.kind() == io::ErrorKind::AlreadyExists {
                Ok(path)
            } else {
                Err(e)
            }
        }
    }
}
