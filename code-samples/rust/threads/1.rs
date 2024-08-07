use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_millis(2000));
        let val = String::from("hi x2");
        tx1.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {received}");
    let received = rx.recv().unwrap();
    println!("Got: {received}");
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {received}");
    }
}
