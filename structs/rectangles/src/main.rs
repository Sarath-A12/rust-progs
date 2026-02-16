// use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

// Method on rectangle
// Can be split into multiple impl blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool { 
        self.width > other.width && self.height > other.height
    }

    //Overloading with attribute name
    // Can be used for getters!
    fn width(&self) -> bool {
        self.width > 0
    }
//Associated functions of Rectangle type
// ALL Functions (even that don't have self as parameter)
// defined in an impl block
// Means that you don't need an instance to call a function
// String::from
    fn square(size: u32) -> Self {
        Self { width:size, height: size, }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2= Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));
    
    println!("rect1 is {rect1:#?}", );

    let square = Rectangle::square(10);
    
}

fn area(rectangle: &Rectangle) -> u32{
    // Accessing fields of a borrowed struct
    // instance does not move the fields!!
    rectangle.width * rectangle.height
}
