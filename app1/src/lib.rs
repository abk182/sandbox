use std::io;
use todos::Todos;

mod dir;
mod todos;

pub fn run() -> Result<(), io::Error> {
    let todos = Todos::new()?;
    todos.add_todo(String::from("awesome todo\nasdasd"))?;
    let todo0 = todos.get_todo(0)?;
    println!("{}", todo0.format());

    Ok(())
}
