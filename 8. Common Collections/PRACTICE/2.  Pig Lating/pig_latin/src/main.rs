use std::io;

fn convert_to_pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut chars = word.chars();
    if let Some(first_char) = chars.next() {
        if vowels.contains(&first_char.to_ascii_lowercase()) {
            return format!("{}-hay", word);
        } else {
            let rest: String = chars.collect();
            let modified_word = format!("{}-{}ay", rest, first_char);
            return modified_word;
        }
    }

    word.to_string()
}

fn convert_sentence_to_pig_latin(sentence: String) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let pig_latin_words: Vec<String> = words
        .iter()
        .map(|word| convert_to_pig_latin(word))
        .collect();

    pig_latin_words.join(" ")
}

fn get_sentence() -> String {
    println!("Enter a sentence:");
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");
    sentence
        .trim()
        .to_string()
        .to_lowercase()
        .replace(".", "")
        .replace(",", "")
        .replace("!", "")
        .replace("?", "")
        .replace("'", "")
}

fn main() {
    let sentence = get_sentence();
    let pig_latin_sentence = convert_sentence_to_pig_latin(sentence);
    println!("{}", pig_latin_sentence);
}
