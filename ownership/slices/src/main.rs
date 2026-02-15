fn main() {
   // Example of invalid program

   let mut s = String::from("hello world");

   let word = first_word(&s);

//    s.clear(); // empties the string. Now it's = ""
// Uncommenting this line would lead to errors!

    // println!("The first word is {word}");
// Remember the previous rule which said you can borrow only mutable reference until
// there are no active immutable references. That.

   // word is still valid here, but s doesn't have any content
   // which could be meaningfully associated with 5. Thus word is invalid

   // Value of first_word isn't associated with
   // Value of the word .In other words, states are not in sync

   //String slices!

   let hello = &s[0..5]; // 0 can be dropped
   let world = &s[6..11]; // 11 can be dropped

   println!("{hello}");

   // When you type the following code 
   /*
   let s = "hello , world!";
   type of s here is &str. It's slice. It's an immutable reference
   Hence it;s value can't be changed
    */

    let  my_string = String::from("hello, world");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    //works on slices of String's, partial or whole

    let word = first_word(&my_string);
    //also works with references to String's which are equivalent
    //to whole slices of String's

    let my_string_literal = "hello world";

    //first_word works on slices of string literals,
    // even if it's partial or whole

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);

}

//Ownership is fine 
// This is better than using &String
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return i; // That was invalid
            return &s[0..i]; 
        }
    }

    &s[..]
}