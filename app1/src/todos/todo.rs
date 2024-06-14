use crate::constants::DATE_FORMAT;
use std::{fmt, fs};
use std::{
    io,
    io::{Read, Write},
};

#[derive(Debug)]
pub struct Todo {
    pub date: chrono::DateTime<chrono::Local>,
    content: String,
}

impl Todo {
    pub fn new(content: &str) -> Todo {
        Todo {
            date: chrono::offset::Local::now(),
            content: String::from(content),
        }
    }

    pub fn read_from_file(file_name: &str) -> Result<Todo, io::Error> {
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

    pub fn delete_file(&self, file_name: &str) -> Result<&Self, io::Error> {
        fs::remove_file(file_name)?;
        Ok(&self)
    }

    pub fn write_to_file(&self, file_name: &str) -> Result<&Self, io::Error> {
        let content = String::from(format!(
            "{}\n{}",
            self.date.format(DATE_FORMAT),
            self.content
        ));
        let mut file = fs::File::create(file_name)?;
        file.write_all(content.as_bytes())?;
        Ok(&self)
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("\nDate: \n{}\n", self.date.format(DATE_FORMAT),));
        output.push_str(&format!("\nContent: \n{}\n", self.content));

        write!(f, "{}", output)
    }
}
