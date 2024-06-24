use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let val = RefCell::new(1);
    
    let x = Rc::new(&val);
    println!("x = {}", x.borrow());
    println!("Rc::strong_count(x) = {}", Rc::strong_count(&x));
    
    *val.borrow_mut() = 2;

    {
        let y = Rc::clone(&x);
        println!("Rc::strong_count(x) = {}", Rc::strong_count(&x));
        let z = Rc::clone(&x);
        println!("Rc::strong_count(x) = {}", Rc::strong_count(&x));

        println!("x = {:?}, y = {:?}, z = {:?}", x.borrow(), y.borrow(), z.borrow());
    }

    println!("Rc::strong_count(x) = {}", Rc::strong_count(&x));
}
