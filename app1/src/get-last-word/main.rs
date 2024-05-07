fn main() {
    let s = String::from("hello beautiful world");

    let w = get_last_word(&s);

    println!("w: {w}")
}

fn get_last_word(s0: &str) -> &str {
    println!("s0: {s0}");
    let bytes = s0.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return get_word(&s0[i+1..]);
        }
    }

    &s0[..]
}