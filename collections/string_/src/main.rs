use std::future::poll_fn;

fn main() {
    // Rust only has str in it's core language!
    // Usually seen in the borrowed form string slice, &str

    let mut s = String::new();

    // Any type that implements the display trait
    // can be converted to String

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    // takes string slice (&str) Reason ?

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // s2 needs to be valid still.  
    println!("s2 is {s2}");

    // Push takes a single character
    let mut s = String::from("lo");
    s.push('l');


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2; // s1 has been moved here and is invalid, s2 is still valid


    // add is self, &str -> String
    /// add takes ownership of self
    // so s1 won't be valid thereafter

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    // Similar to println!...instead of printing to terminal it returns a string

    // ALL parameters are still valid!


    // Indexing...tricky part
    // let t = s1[0];
    // Never index into strings - only slice into valid byte arrays

    // Use string.chars() or string.bytes()
    for ch in "नमस्ते".chars() {
        println!("{ch}");
    }
}
