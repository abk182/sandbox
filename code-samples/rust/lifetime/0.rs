fn main() {
    let string1;
    let mut string2; 
    string1 = String::from("long string is long");
    let result;

    {
        string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("The longest string is {}", result);
    }

    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}