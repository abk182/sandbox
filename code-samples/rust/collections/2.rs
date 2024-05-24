fn main() {
    println!("{}", convert_phrase_to_pig_latin(String::from("hello awesome world")));
    println!("{}", convert_phrase_to_pig_latin(String::from("привет успешные")));
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
    let consonants = vec!['h', 'w', 'п'];
    let vowels = vec!['a', 'у'];
    let mut pig_latin_word = String::new();
    let mut letters = word.chars();
    let first_letter = letters.next().unwrap();

    if vowels.contains(&first_letter) {
        pig_latin_word.push_str(&format!("{}-hay", word))
    }

    if consonants.contains(&first_letter) {
        for letter in letters {
            pig_latin_word.push(letter)
        }
        pig_latin_word.push_str(&format!("-{}ay", first_letter))
    }

    pig_latin_word
}
