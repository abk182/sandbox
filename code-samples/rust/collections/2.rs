fn main() {
    let str = String::from("hello awesome world");

    println!("{}", convert_phrase_to_pig_latin(str));
}

fn convert_phrase_to_pig_latin(phrase: String) -> String {
    let mut output = String::from("");

    for word in phrase.split_whitespace() {
        output.push_str(&format!(
            "{} ",
            convert_word_to_pig_latin(String::from(word))
        ))
    }

    output
}

fn convert_word_to_pig_latin(word: String) -> String {
    let consonants = vec!["H", "w"];
    let vowels = vec!["a"];
    let first_letter = &word[..1];
    let mut pig_latin_word = String::new();
    println!("{first_letter}");

    if vowels.contains(&first_letter) {
        pig_latin_word.push_str(&format!("{}-hay", &word))
    }

    if consonants.contains(&first_letter) {
        pig_latin_word.push_str(&format!("{}-{}ay", &word[1..], &word[..1]))
    }

    pig_latin_word
}
