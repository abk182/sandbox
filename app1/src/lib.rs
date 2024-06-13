use std::io::{self, Write};
use todos::Todos;

mod constants;
mod utils;
mod todos;

pub fn run() -> Result<(), io::Error> {
    let mut input = String::new();
    let todos = Todos::new()?;

    loop {
        input.clear();
        print!("command (add|get|exit): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let command = input.trim();

        match command {
            "get" => {
                input.clear();
                print!("get: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut input)?;

                let id = input.trim().parse::<i32>().unwrap();
                let todo = todos.get_todo(id)?;
                println!("{}", todo);
            }
            "add" => {
                input.clear();
                print!("add: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut input)?;

                // todo: fix .clone()
                todos.add_todo(String::from(input.clone()))?;
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
