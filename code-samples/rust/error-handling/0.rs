use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

fn main_v2() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];
    let greeting_file = File::open("hello.txt");

    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file = File::open("hello.txt").expect("no hello.txt");

    let greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => {
                    panic!("Error creating file: {:?}", error)
                }
            },
            other_error => {
                panic!("other error: {:?}", other_error)
            }
        },
    };
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let text = read_username_from_file();
    let text = read_username_from_file_v2();
    let text = read_text_from_file().unwrap();
    let last_char = last_char_of_first_line(&text).unwrap();
    println!("{}", last_char);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => return Ok(username),
        Err(e) => return Err(e),
    }
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_text_from_file() -> Result<String, io::Error> {
    match File::open("hello.txt") {
        Ok(mut file) => {
            let mut s = String::new();

            match file.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
