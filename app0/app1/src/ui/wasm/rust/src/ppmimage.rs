use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn draw_ppm_image(width: u16, height: u16) -> String {
    let mut output = String::from("");

    output.push_str(&format!("P3\n{} {}\n255\n", width, height));

    for j in (0..height).rev() {
        for i in 0..width {
            let r = i as f64 / (width - 1) as f64;
            let g = j as f64 / (height - 1) as f64;
            let b = 0.25;

            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;
            

            output.push_str(&format!("{} {} {}\n", ir, ig, ib));
        }
    }
    
    output
}
