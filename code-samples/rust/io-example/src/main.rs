use std::{process, env};

use app1::{Config, run};
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Problem parsing arguments: {e}");
        process::exit(1);
    }
}


