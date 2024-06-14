use std::io::{self, Write};
use todos::Todos;

mod constants;
mod todos;
mod utils;

pub fn run() -> Result<(), io::Error> {
    let mut input = String::new();
    let todos = Todos::new()?;

    loop {
        input.clear();
        print!("command (get|add|delete|find|exit): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let command = input.trim();

        match command {
            "get" => {
                input.clear();
                print!("todo_id: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut input)?;

                let id = input.trim().parse::<i32>().unwrap();
                let todo = todos.get_todo(id)?;
                println!("{}", todo);
            }
            "add" => {
                input.clear();
                print!("content: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut input)?;

                let todo = todos.add_todo(&input)?;
                println!("added {:?}", todo);
            }
            "delete" => {
                input.clear();
                print!("todo_id: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut input)?;

                let id = input.trim().parse::<i32>().unwrap();
                let todo = todos.delete_todo(id)?;
                println!("deleted {:?}", todo);
            }
            "find" => {
                input.clear();
                print!("date_from: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut input)?;
                let from = input.clone();

                input.clear();
                print!("date_to: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut input)?;
                let to = input.clone();

                let list = todos.search_todo_in_date_range(&from, &to)?;
                println!("{:?}", list)
            }
            "exit" => {
                println!("exited");
                return Ok(());
            }
            _ => {
                println!("try again");
            }
        }
    }
}
