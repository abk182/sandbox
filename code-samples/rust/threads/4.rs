use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;


fn main() {
    let x = Rc::new(RefCell::new(5));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // let y = Rc::clone(&x);
        println!("{counter:?}");
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            println!("num: {num}");
            // println!("num: {y:?}");

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}