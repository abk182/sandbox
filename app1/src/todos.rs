use todo::Todo;

use crate::dir::use_dir;
use std::fs;
use std::io;

mod todo;

#[derive(Debug)]
pub struct Todos {
    dir: String,
    extension: String,
}

impl Todos {
    pub fn new() -> Result<Todos, io::Error> {
        let dir = use_dir(None)?;
        let todos = Todos {
            dir,
            extension: String::from("txt"),
        };

        println!("{:?}", todos);

        Ok(todos)
    }

    pub fn add_todo(&self, text: String) -> Result<(), io::Error> {
        let todo = Todo::new(text);
        todo.write_to_file(self.file_name()?)?;
        Ok(())
    }

    pub fn get_todos(&self, date_from: &str, date_to: &str) {
    }

    fn file_name(&self) -> Result<String, io::Error> {
        Ok(format!(
            "{0}/{1}.{2}",
            &self.dir,
            self.generate_todo_id()?,
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

    fn generate_todo_id(&self) -> Result<i32, io::Error> {
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
