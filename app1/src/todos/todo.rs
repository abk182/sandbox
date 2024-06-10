use std::fs;
use std::io;
use std::io::Write;

#[derive(Debug)]
pub struct Todo {
    date: chrono::DateTime<chrono::Local>,
    content: String,
}

impl Todo {
    pub fn new(content: String) -> Todo {
        Todo {
            date:  chrono::offset::Local::now(),
            content
        }
    }

    pub fn write_to_file(&self, file_name: String) -> Result<&Self, io::Error> {
        let content = String::from(format!("{}\n{}", self.date, self.content));
        let mut file = fs::File::create(file_name)?;
        file.write_all(content.as_bytes())?;
        Ok(&self)
    }
}