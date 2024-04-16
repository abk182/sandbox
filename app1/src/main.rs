fn main() {
    fail_hello_world()
}

fn fail_hello_world() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);
    println!("{}, world!", s2);
}