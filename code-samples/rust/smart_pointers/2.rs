enum List<'a> {
    Cons(i32, Box<&'a List<'a>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let binding = Cons(10, Box::new(&Nil));
    let a = Cons(5, Box::new(&binding));
    {
        let b = Cons(3, Box::new(&a));
    }
    let c = Cons(4, Box::new(&a));
}