mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: String);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    let mut output = String::from("Hello, ");
    output.push_str(s);
    output.push_str(" !");
    alert(output);
}
