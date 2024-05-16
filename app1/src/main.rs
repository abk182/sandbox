fn main() {
    let rect = Rectangle {
        width: 11,
        height: 2
    };
    println!("area: {0}", area(rect));
}

fn area(dimensions: Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

struct Rectangle {
    width: u32,
    height: u32,
}