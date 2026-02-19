enum Message {
    Quit,
    Move { x: i32, y:i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Enums also have impl!!

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
impl Message {
    fn call(&self) {
        //method body
    }
}

enum Coin {
    Penny,
    Nickel, 
    Dime, 
    Quarter(UsState),
}



fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1, // Comma optional if multiple statements and braces
        Coin::Nickel => {
            println!("Lucky nickel");
            5
        } 
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
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

    // let sum = x + y;
    let sum = x + match y {
        Some(i) => i,
        None => 0
    };

    println!("Sum is {}", sum);
    // Match construct for ENUMS

    let val = value_in_cents(Coin::Quarter(UsState::Alaska));


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    //
    //REMEMBER: Matches are exhaustive. It's supposed to be exhaustive!!

    // Wildcard entries

    let dice_roll = 9;
    match dice_roll {
        3 => 3,
        7 => 7,
        other => other + 1 // wildcard which takes any other number
        // _ => () do nothing
    }
    
}
