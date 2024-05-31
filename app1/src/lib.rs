use std::io;
use todo::Todo;

mod dir;
mod todo;

pub fn run() -> Result<(), io::Error> {
    let todo = Todo::new()?;
    todo.add_todo(String::from("awesome todo"))?;

    Ok(())
}
