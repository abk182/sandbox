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
        let new_todo_id = self.generate_new_todo_id()?;
        let file_name = format!("{}/{}.{}", &self.dir, new_todo_id, &self.extension);
        let mut file = fs::File::create(file_name)?;
        file.write_all(text.as_bytes())?;
        Ok(())
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

    fn generate_new_todo_id(&self) -> Result<i32, io::Error> {
        let mut list = self.list_files()?;
        list.sort();


        if list.len() > 0 {
            let extension = String::from(format!(".{}",self.extension));
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
