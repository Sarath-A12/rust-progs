#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel, 
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if(state.existed_in(1900)) {
            Some(format!("{state:?} is pretty old, for America!"))
        }
        else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
// Another way 

fn describe_state_quarter1(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin { 
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

// Using let - else
fn describe_state_quarter3(coin: Coin) -> Option<String> {
    // Pattern matching happening in the first itself
    let Coin::Quarter(state) = coin else {
        return None;
    };
    
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }

}


fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is confiigured to be {max}"),
        _ => (), // Annoying boilerplate
    }

    // We just need to execute a code in some condition
    // We don't have to specify the wildcard (I mean we do have to , but can we avoid it ? )

    // Alternatively
    // let config_max: Option<i32>  = None;

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    else {
        println!("Value called with None!");
    }
    // Pattern and an expression separated by '='
    // Losing exhaustive checking
    // Syntactic sugar for when pattern matches one arm and ignores the resT!
    let coin = Coin::Quarter(UsState::Alabama);

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
    let coin = Coin::Quarter(UsState::Alaska);
    
    // Alternatively - using an if let
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    }
    else {
        count += 1;
    }

    // Happy Path - Perform computation when value is present
    // Otherwise return default value
}


// Key takeaway - if logic is too verbose in match
// Use if let or let ... else