/*
Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word 
and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end 
instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
*/

use std::io::stdin;
fn main() {
    println!("Enter the string to convert to piglatin");
    let s = input_string();

    // use split whitespaces for better readability
    for word in s.split_whitespace() {
        print!("{} ", convert_word_to_piglatin(word));
    }
}

fn convert_word_to_piglatin (s: &str) -> String {
    let mut s_str = String::new();
    if let Some(ch) = s.chars().nth(0) {
        if !is_vowel(ch) {
            // println!("The piglatin is {}{}ay", &s[1..], ch);
            s_str.push_str(&s[1..]);
            s_str.push(ch);
            s_str.push_str("ay");
        } else {
            // println!("The piglatin is {}hay", s);
            s_str.push_str(s);
            s_str.push_str("hay");
        }
    }
    s_str.to_string()
}

fn input_string() -> String {
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Error occured");
    s = s.trim().to_string();
    s.push(' ');
    s
}


fn is_vowel(c: char) -> bool {
    match c {
        'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
