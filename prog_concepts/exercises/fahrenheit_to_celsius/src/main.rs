use std::io;
// Use this to convert F to C.
// C = (F-32) * 5/9
fn main() {
    println!("Enter the value in fahrenheit");
    let mut input = String::new();

     io::stdin()
            .read_line(&mut input)
            .expect("Error occured");
    
    let fahr: f32= input.trim().parse().expect("Unknown value");

    let celsius= (fahr - 32.0) * 5.0/9.0;
    println!("The value in celsius is {}", celsius);
}
