

// Convert strings to pig latin. The first consonant of each 
// word is moved to the end of the word and ay is added, so first 
// becomes irst-fay. Words that start with a vowel have hay added to the 
// end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!


use std::{io, env};

fn pig_latin(input: &str) -> String {
    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let mut pig_word = String::new();

    for word in input {
        let mut chars = word.chars();

        if let Some(first_char) = chars.next() {
            match first_char.to_lowercase().next() {
                Some('a' | 'e' | 'i' | 'o' | 'u') => {
                    pig_word += &format!("{}-hay ", word);
                },
                Some(consonant) => {
                    pig_word += &format!("{}-{}ay ", chars.collect::<String>(), consonant);
                },
                _ => pig_word += word
            }

        }
    }
    pig_word
}

fn main() {
    // let sentence = "abb Hello World Computer Programming Artificial intelligence Science Technology Future Humanity Life";

    env::set_var("TITLE", "My Program");

    println!("Translation of sentences into pig Latin!\nexample:\n'apple' will be 'applehay'\n'hello world' will become 'ellohay orld-way'");
    println!("");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // println!("{}", pig_latin(&sentence));
        println!("{}", pig_latin(&input));

        println!("-----")
    }

}