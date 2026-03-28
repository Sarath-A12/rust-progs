// struct defined with multiple generic type
struct Point<T, U> {
    x: T,
    y: U,
}
impl Point<f32, f32> {
    fn ans(&self) -> f32 {
        self.x + self.y
    }
}
//can be used on different types as well <X1, Y1> <X2, Y2>, which

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// function defined with generic type
fn largest<T: std::cmp::PartialOrd>(number_list: &[T]) -> &T {
    let mut largest = &number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number in list 1 is {}", result);
    // println!("The largest number is {}", largest);

    let char_list = vec!['a', 'j', 'k', 'l', 'z', 'a'];
    let result = largest(&char_list);

    let integer = Point { x: 5, y: 10 };
    let floating = Point { x: 5.2, y: 10.5 };
    let both = Point { x: 5, y: 10.5 };

    println!("The largest number in list 1 is {}", result);
    println!("The value of x is {}", both.x());
}
