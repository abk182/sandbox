mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn calc(current_income: f32, years: u8, inflation: f32) -> f32 {
    let mut current_year = 0;
    let mut calculated_income = current_income;
    while current_year < years {
        calculated_income = calculated_income * (1.0 + inflation);
        current_year += 1;
    }

    calculated_income
}
