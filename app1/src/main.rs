fn main() {
    let rect = (2, 3);
    println!("area: {0}", area(rect));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}