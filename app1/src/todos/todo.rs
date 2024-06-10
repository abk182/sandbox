use std::fs;
use std::io;
use std::io::{Read, Write};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S %z";

#[derive(Debug)]
pub struct Todo {
    date: chrono::DateTime<chrono::Local>,
    content: String,
}

impl Todo {
    pub fn new(content: String) -> Todo {
        Todo {
            date: chrono::offset::Local::now(),
            content,
        }
    }

    pub fn read_from_file(file_name: String) -> Result<Todo, io::Error> {
        let mut content = String::new();
        fs::File::open(file_name)?.read_to_string(&mut content)?;
        let (date, content) = content.split_once("\n").unwrap();
        Ok(Todo {
            date: chrono::DateTime::parse_from_str(date, DATE_FORMAT)
                .unwrap()
                .into(),
            content: String::from(content),
        })
    }

    pub fn write_to_file(&self, file_name: String) -> Result<&Self, io::Error> {
        let content = String::from(format!(
            "{}\n{}",
            self.date.format(DATE_FORMAT),
            self.content
        ));
        let mut file = fs::File::create(file_name)?;
        file.write_all(content.as_bytes())?;
        Ok(&self)
    }

    pub fn format(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("\nDate: \n{}\n", self.date.format(DATE_FORMAT),));
        output.push_str(&format!("\nContent: \n{}\n", self.content));
        output
    }
}
