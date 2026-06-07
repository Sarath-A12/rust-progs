//cons list (1, (2, (3, Nil)))
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
fn main() {
    let b = Box::new(5); //stores data on heap (malloc() equivalent ? )
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
