fn main() {
    let mut v1: Vec<String> = Vec::new();
    v1.push(String::from("abc"));
    let mut v2 = vec![1, 2, 3];

    // break
    // let abc = v1[0];

    // break
    // let first = &v2[0];
    // v2.push(6);
    // print!("{first}");

    for i in &mut v2 {
        *i += 50;
        println!("{i}");
    }

    println!("{:?},{:?}", v1[0], v2.get(1));

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let hello = "ывфаф";
    let s = &hello[0..4];
    println!("{s}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Рщш".bytes() {
        println!("{b}");
    }
}
