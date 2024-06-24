use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    let y = &x;
    let z = &x;

    *x.borrow_mut() += 10;

    println!("{:?}", y);
    println!("{:?}", z);
}