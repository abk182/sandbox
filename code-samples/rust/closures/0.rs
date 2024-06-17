use std::thread;

fn main() {
    let some_str = "some str";
    let maybe_foo: Option<&str> = None;

    let foo = maybe_foo.unwrap_or_else(|| &some_str);

    println!("{foo}");

    // break;
    // fn baz () {
    //     println!("{}", foo);
    // }

    // baz();

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    println!("Before defining closure: {list:?}");
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {list:?}");
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    // println!("After thread: {list:?}");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    let mut sort_operations: Vec<String> = vec![];
    let value = String::from("closure called");
    list.sort_by_key(|r| {
        println!("{:?}", sort_operations);
        println!("{:?}", value);
        // sort_operations.push(value);
        sort_operations.push(String::from("closure called"));
        r.width
    });
    println!("{list:#?}");
}

fn take_ownership<T: std::fmt::Debug>(t: T) {
    println!("{:?}", t);
}
