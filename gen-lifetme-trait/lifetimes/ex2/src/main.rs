fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // This is fine
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("cyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    //This is not

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("cyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
    // In some sense - result would be valid as long as BOTH string1 and string2 are valid
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// For some lifetime 'a the function takes two parameters, both of which
// are string slices that live at least as long as lifetime 'a. The function
// signature also says that the returned string slice will also live at least
// as long as lifetime 'a .
//
// The lifetime of the reference returned by the longest function
// would be the smaller of the lifetime of the values referred by the lifetime
// of the function arguments
//
//
// This says that the returned reference would be valid as long
// as both of the parameters are valid
//
// Say we have another function that looked like this
fn longest_other<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// Means it's valid as long as just x is valid
//
// This won't compile

fn longest_ot<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    // result.as_str()
    // creates dangling pointer as the reference is invalid at the end
    // of this function
    result
}
