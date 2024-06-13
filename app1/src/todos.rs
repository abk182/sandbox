use crate::{utils::parse_with_multiple_formats};
use crate::utils::use_dir;
use std::{fs, io};
use todo::Todo;

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
        Ok(todos)
    }

    pub fn add_todo(&self, text: String) -> Result<(), io::Error> {
        let todo = Todo::new(text);
        todo.write_to_file(&self.file_path_by_id(self.generate_todo_id()?))?;
        Ok(())
    }

    pub fn get_todo(&self, id: i32) -> Result<Todo, io::Error> {
        let str = Todo::read_from_file(&self.file_path_by_id(id))?;
        Ok(str)
    }

    pub fn search_todo_in_date_range(
        &self,
        date_from: &str,
        date_to: &str,
    ) -> Result<Vec<Todo>, io::Error> {
        let mut output: Vec<Todo> = vec![];
        let from = parse_with_multiple_formats(date_from)?.timestamp();
        let to = parse_with_multiple_formats(date_to)?.timestamp();

        for file in self.list_files()? {
            let todo = Todo::read_from_file(&format!("{0}/{1}", &self.dir, file))?;
            let now = todo.date.timestamp();
            if now > from && now < to {
                output.push(todo);
            }
        }

        Ok(output)
    }

    fn file_path_by_id(&self, id: i32) -> String {
        format!("{0}/{1}.{2}", &self.dir, id, &self.extension)
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
