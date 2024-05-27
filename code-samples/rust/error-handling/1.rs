fn main() {
    match try_to_panic(true) {
        Ok(res) => {
            println!("{}, no panic...", res)
        }
        Err(e) => {
            println!("{}! panic!!!", e)
        }
    }
}

fn try_to_panic(should_panic: bool) -> Result<String, String> {
    if should_panic {
        Err(String::from("should panic"))
    } else {
        Ok(String::from("should not panic"))
    }
}
