fn main() {
    let rect = Rectangle {
        width: 11,
        height: 2,
        area: 23
    };

    dbg!(&rect);
    println!("rect {:#?}", rect);
    println!("area: {0}", rect.area());
    println!("area: {0}", (&rect).area());
    println!("area {0}", if rect.check_area() {"ok"} else {"not ok"});
    rect.take_ownership_of_self();
    // rect.take_ownership_of_sefl(); // wont work, borrowing
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

    fn take_ownership_of_self(self) -> u32 {
        self.area()
    }

}