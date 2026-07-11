use std::ops::Deref;

//cons list (1, (2, (3, Nil)))
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
use crate::List::{Cons, Nil};
fn main() {
    let b = Box::new(5); //stores data on heap (malloc() equivalent ? )
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // it runs *(y.deref())  BTS

    //Effectively same
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    //Using newly constructed MyBox

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // deref coercion  - converts &MyBox<String> -> &String -> &str
    //

    //Alternatively
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]); // *m converts to &String , and & .. converts it to &str (the whole string)

    // Deref for immutable
    // DerefMut for mutable
    // Mutable references can be converted to immutable - never vice versa (borrowing issues)

    // Drop

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(d); // Force drop

    println!("CustomSmartPointers created");
}

fn hello(name: &str) {
    println!("Hello, {name}");
}
