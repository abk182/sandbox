use crate::dir::use_dir;
use std::fs;
use std::io;
use std::io::Write;

#[derive(Debug)]
pub struct Todo {
    dir: String,
    extension: String,
}

impl Todo {
    pub fn new() -> Result<Todo, io::Error> {
        let dir = use_dir(None)?;
        let todo = Todo {
            dir,
            extension: String::from("txt"),
        };

        println!("{:?}", todo);

        Ok(todo)
    }

    pub fn add_todo(&self, text: String) -> Result<(), io::Error> {
        let timestamp = chrono::offset::Local::now().timestamp();
        let content = String::from(format!("{}\n{}", timestamp, text));
        let mut file = fs::File::create(self.file_name()?)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn get_todo(&self, date_from: &str, date_to: &str) {
    }

    fn file_name(&self) -> Result<String, io::Error> {
        Ok(format!(
            "{}/{}.{}",
            &self.dir,
            self.new_todo_id()?,
            &self.extension
        ))
    }

    fn list_files(&self) -> Result<Vec<String>, io::Error> {
        let mut vector: Vec<String> = vec![];
        let entities = fs::read_dir(&self.dir)?;

        for entity in entities {
            let file_name = entity?
                .file_name()
                .into_string()
                .unwrap_or_else(|err| panic!("can't convert file name into string. {:?}", err));

            vector.push(file_name);
        }

        Ok(vector)
    }

    fn new_todo_id(&self) -> Result<i32, io::Error> {
        let mut list = self.list_files()?;
        list.sort();

        if list.len() > 0 {
            let extension = String::from(format!(".{}", self.extension));
            let mut last_id: i32 = list[list.len() - 1]
                .replace(&extension, "")
                .parse()
                .unwrap_or_else(|err| panic!("cant get last todo id. {:?}", err));
            last_id += 1;

            Ok(last_id)
        } else {
            Ok(0)
        }
    }
}
