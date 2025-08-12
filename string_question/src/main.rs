use std::io::{self, Write};

fn main() {
    /*
    Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added,
    so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
    Keep in mind the details about UTF-8 encoding!
         */

    // get the string input:
    print!("Enter the word you want to convert pig latin: ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // to get rid of the new line

    // handling vowels:
    let is_vowel: bool;
    let s_first = s.chars().nth(0);
    match s_first {
        Some('a') | Some('e') | Some('i') | Some('o') | Some('u') | Some('A') | Some('E')
        | Some('I') | Some('O') | Some('U') => is_vowel = true,
        _ => is_vowel = false,
    }

    //printing:
    if is_vowel {
        let pig_lang = format!("{}-hay", s);
        println!("{pig_lang}");
    } else {
        let pig_lang = format!("{}-{}ay", &s[1..], &s[0..1]);
        println!("{pig_lang}");
    }
}
