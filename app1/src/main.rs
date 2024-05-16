fn main() {
    let rect = Rectangle {
        width: 11,
        height: 2
    };

    dbg!(&rect);
    println!("rect {:#?}", rect);
    println!("area: {0}", area(&rect));
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}