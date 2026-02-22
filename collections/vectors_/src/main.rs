// Vectors

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let mut v = vec![1, 2, 3];

    // Alternatively 
    // let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element!"),
    }

    // Choose between both based on whether you want the program to crash or continue
    // Borrow checker rules are valid here

    let third: &i32 = &v[2];
    println!("The thid element is {}", third);
    v.push(6);
    // When you push an element, extra space may not be there
    // So new space allocated to the ENTIRE vector, thereby causing the 
    // old reference point to be invalid !

    // println!("The third element is: {third}");
    for i in &v {
        println!("{i}");
    }

    // Mutably we can do it as well
    for i in &mut v {
        *i += 50;
    }

    // Vectors restricted to same types
    // For multiple types use enums !

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("Blue")),
        SpreadSheetCell::Float(10.12),
    ];

} // all vectors go out of scope here - so they are dropped!
