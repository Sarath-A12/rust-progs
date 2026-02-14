fn main() {
    let s = String::from("Hello");
    //s comes into scope

    takes_ownership(s);
    // s transferred ownership to the function
    // s is no longer valid after this
    // println!("After call: {}", s); 
    // Above line won't work

    let x = 5;
    // x comes into scope

    makes_copy(x);
    // x implements Copy trait
    // x does not MOVE, but COPIES
    println!("After call: {}",x );


    //Similarly, returning a function value
    // Also transfers ownerhsip

    let s1 = gives_ownership();
    // gives_ownership moves it's return value into s1

    println!("{} still exists", s1);
    let s2 = String::from("Hello");
    // s2 comes into scope

    let s3 = takes_and_gives_back(s2);
    // s2 moves to the fn, which moves
    //it's return value is gone to s3
    // so s2 lives via s3
    // println!("{} still exists", s2);
    println!("{} still exists", s3);
    
    
}
// s3 goes out of scope. s2 was moved already
// s1 is also dropped
// x goes out of scope, so does s
// but s is already invalid so no worries

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
    // this will move it's return value
    // into the function that calls it
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
    // a_string comes into scope and 
    // is returned and moves into 
    // the calling function
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope here
    println!("{some_string}");
}
// some_string goes out of scope and 'drop' is called
// whatever memory was pointed to by 
// some_string, which would be s
// is now freed and invalid

fn makes_copy(some_integer: i32){
    println!("{some_integer}");
}
// some integer goes out of scope. 
// but it's a new value
