use std::io;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//Needs to be set at compile time, and type specified
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
    // possible to change type
    let spaces = "  ";
    let spaces = spaces.len();

    /*
    //will throw error
    let mut spaces = "   ";
    spaces = spaces.len();
    */
    let tup : (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("The value of z is {}", tup.2);
    
    // Tuples can have different types -
    // Arrays need to have all elements same type
    // Arrays need to be of fixed length :)

    let arr = [1, 2, 3, 4, 5];

    // Use type; number of elements

    let arr1 : [i32; 5] = [1, 2, 3, 4, 5];

    // To fill with default values

    let arr2 = [3;5];
    // println!("{}", arr2);

    let first = arr[0];

    println!("Enter the element to access");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];
    println!("The value of element at index {} is {}", element, index);
}
