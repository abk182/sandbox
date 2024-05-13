mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn calc(currentIncome: f32, years: u8, inflation: f32) -> f32 {
    let mut currentYear = 0;
    let mut calculatedIncome = currentIncome;
    while currentYear < years {
        calculatedIncome = calculatedIncome * (1.0 + inflation);
        currentYear += 1;
    }

    calculatedIncome
}
