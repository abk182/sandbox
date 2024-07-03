use std::thread;
use std::time::Duration;

fn main() {
    let list = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("list from thread: {list:?}");
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // println!("list from thread: {list:?}"); list moved


    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    println!("END");
}
