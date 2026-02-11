fn main() {
    println!("Hello, world!");
    
    another_function(5, 'h');
    
    //statements - no return value
    /*
        let y = 6; // statement
        let y = (let x = 6); //error because statement don't return values like in C
    */
    //expressions - return value

    let y = {
        let x = 3;
        x+1
    }; // is an expression which evaluates to 4
    //ERROR if you put semicolon in the end - where it becomes a statement!

    let x = five();
    println!("The value of x is: {}", x);

    let y = plus_one(5);
    println!("The value of y is {}", y);

    if y < 3 {
        println!("The condition was true");
    }
    else {
        println!("The condition was false");
    }
    

    // IF IS AN EXPRESSION MEANS 
    // IT CAN BE ON THE RHS

    let condition = true;
    let number = if condition {5} else {6};
    // Strong type checking between different arms. will throw error if not the right type


    println!("The value of number is {}", number);


    let mut counter = 0;

    let result = loop {
        // println!("again!");
        counter+=1;

        if counter == 10 { 
            break counter * 2;
        }

    };
    
    println!("the result of the loop is {}", result);

    //LOOP LABELS

    //By default they break out of the inner loop
    // Using labels break out of any loop
    let mut count = 0; 
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2{
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    //WHILE Looops as usual

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    
    // Alternatively (since this is time consuming and error prone)

    for element in a {
        println!("Value using this is: {}", element);
    }

    for number in  (1..4).rev() {
        println!("{number}");
    }
}

fn another_function(x: i32, label: char) {
    println!("The value of x is: {} in {}", x, label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) ->i32 {
    // throws error : x + 1;
    x + 1
}