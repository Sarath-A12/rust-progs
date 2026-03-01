use core::panic;
use std::io;

/*
Given a list of integers, use a vector and return the median
(when sorted, the value in the middle position) and mode
(the value that occurs most often; a hash map will be helpful here)
of the list.
*/
fn main() {
    // Get input from the user
    let input = get_list_from_user();

    let mut input_copy = input.clone();
    let median: i32 = find_median(&mut input_copy);
}

fn get_list_from_user() -> Vec<i32> {
    let mut list = Vec::new();
    let mut input_string = String::new();

    io::stdin().read_line(&mut input_string).expect("Invalid input");
    let input_size: i32 = input_string.trim().parse()
                                        .expect("Invalid number entered");
    for _ in 1..input_size {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Invalid input");
        let num_int: i32 = num.trim().parse().expect("Invalid number");
        list.push(num_int);
    }

    list
}


fn find_median(mut input: Vec<i32>) -> i32 {
    input.sort();
    let mut ans: f64 = -1.0;
    let size = input.len();
    if size%2 == 0 {
        if let ans =  {
            
        }
    }
    else {
        ans = match input.get((size - 1)/ 2) {
            Some(val) => *val.into(), 
            None => panic!(
                "Unknown error occured"
            ),
        };
    }

    ans
}