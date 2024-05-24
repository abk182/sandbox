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

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Yellow")).or_insert(99);
    scores.entry(String::from("Blue")).or_insert(99);

    println!("{:?}", scores);
    
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    println!("score: {}", scores[&String::from("Yellow")]);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {score}");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // break
    // println!("{field_name}, {field_value}")


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);    
}
