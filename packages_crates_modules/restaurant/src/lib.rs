// Struct - members have to be made public
// Enums, all are public by default

use std::collections::HashMap;

// Nested paths to clean up use lists

// Rather than
// use std::cmp::Ordering;
// use std::io;

// We use
use std::{cmp::Ordering, io};

/*
GLOB OPERATOR (*)
 */

 use std::collections::*;
 // Brings all public items defined in std::collections into current scope!

// We may also use
// use std::io::{self, Write} to import std::io and std::io::Write;

mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast { //struct is public
        pub toast: String, // field is public
        seasonal_fruit: String, // field is private

    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("watermelon"),
            }
        }
    }

    fn cook_order() {}
}

// USE KEYWORD TO BRING ITEMS INTO SCOPE
pub use crate::front_of_house::hosting;
// Only works if you use it in places where this scoope is valid
// Won't work if you use it with a different module, because that' s adifferent scope ?

// Instead use : pub use so that it can be accessed from anywhere
// Called re exporting, because also it exports to that module


// Idiomatic paths
/*
We don't bring the entire function into path, but just upto the parent ,this is primarily because
we don't want any confusions as to if the function is defined locally or if it's some other external one
For structs and enums it's okay!
*/
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Using the use keyword's power
    hosting::add_to_waitlist();

    // Rye toast order
    let mut meal = back_of_house::Breakfast::summer("Rye");

    //Whoops, im allergic
    meal.toast = String::from("Wheat"); //acceptable because field is public
    println!("I'd like {} toast please", meal.toast);

    // Won't work
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Won't work without pub use
    hosting::add_to_waitlist();
}

// Issue with name clashes
// Either specify every time
// Or use as keyword to specify properly

use std::fmt;
// use std::io;

use std::fmt::Result; // can be referred to as Result
use std::io::Result as IoResult; // Will be referered to as IoResult
// fn function1() -> fmt::Result {

// }

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
