pub mod calc;
pub mod ppmimage;
mod utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
