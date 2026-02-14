//Prints the nth fib
use std::{io::{stdin}};
fn main() {
    println!("Enter the number n");
    let mut input = String::new();

    stdin()
    .read_line(&mut input)
    .expect("Error occured while parsing");

    let n: i32 = input
                    .trim()
                    .parse()
                    .expect("Error occured");

    let mut a1 = 1;
    let mut a2 = 1;
    for _ in 1..n {
        let a3 = a1 + a2;
        a1 = a2;
        a2 = a3;
    }
    println!("The {}th fib is {}",n, a1);

}
