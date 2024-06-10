use std::io;
use todos::Todos;

mod dir;
mod todos;

pub fn run() -> Result<(), io::Error> {
    let todos = Todos::new()?;
    todos.add_todo(String::from("awesome todo"))?;
    todos.get_todos("10.06.2024T23:59", "10.06.2024T00:00");

    Ok(())
}
