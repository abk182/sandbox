enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));
    
    let a_rc = Rc::new(a);
    println!("count after creating a_rs = {}", Rc::strong_count(&a_rc));
    let b = Cons(3, Rc::clone(&a_rc));
    println!("count after creating b = {}", Rc::strong_count(&a_rc));
    {
        let c = Cons(4, Rc::clone(&a_rc));
        println!("count after creating c = {}", Rc::strong_count(&a_rc));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a_rc));
}