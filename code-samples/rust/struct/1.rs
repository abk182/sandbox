fn main() {
    let rect0 = Rectangle {
        width: 11,
        height: 2,
        area: 22,
    };
    dbg!(&rect0);
    println!("rect {:#?}", rect0);
    println!("area: {0}", rect0.area());
    println!("area: {0}", (&rect0).area());
    println!("area {0}", if rect0.check_area() { "ok" } else { "not ok" });
    // rect0.take_ownership_of_self(); // break here
    // rect0.take_ownership_of_sefl(); // wont work, borrowing

    let rect1 = Rectangle {
        width: 10,
        height: 1,
        area: 10,
    };
    println!(
        "rect0 {0} rect1",
        if rect0.can_hold(&rect1) {
            "can hold"
        } else {
            "can not hold"
        }
    );

    let square = Rectangle::square(5);
    dbg!(&square);

    // let square_from_rect = rect1.square(5);
    // dbg!(&square_from_rect);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    area: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn check_area(&self) -> bool {
        self.area() == self.area
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn take_ownership_of_self(self) -> u32 {
        self.area()
    }
}

impl Rectangle {
    fn square(length: u32) -> Self {
        Rectangle {
            width: length,
            height: length,
            area: length * length,
        }
    }
}
