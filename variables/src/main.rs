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
}
