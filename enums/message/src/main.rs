enum Message {
    Quit,
    Move { x: i32, y:i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Enums also have impl!!

impl Message {
    fn call(&self) {
        //method body
    }
}

enum Coin {
    Penny,
    Nickel, 
    Dime, 
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1, // Comma optional if multiple statements and braces
        Coin::Nickel => {
            println!("Lucky nickel");
            5
        } 
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
    // OPTIONS

    let some_number = Some(5);
    let some_char = Some('e');
    
    let absent_number: Option<i32> = None;


    // Why options provide null safety ?

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x+ y;

    // Match construct for ENUMS
}
