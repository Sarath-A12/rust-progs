use std::{collections::HashMap, io};

/*
Given a list of integers, use a vector and return the median
(when sorted, the value in the middle position) and mode
(the value that occurs most often; a hash map will be helpful here)
of the list.
*/
fn main() {
    // Get input from the user
    let input = get_list_from_user();

    let median= find_median(input.clone());
    println!("The median is {}", median);
    // println!("Sanity check if input_copy is valid: {:?}", input_copy);

    let mode = find_mode(input);
    println!("The mode of the array is {}", mode);
}

fn get_list_from_user() -> Vec<i32> {
    let mut list = Vec::new();
    let mut input_string = String::new();

    println!("Enter the number of elements in the vector");
    io::stdin().read_line(&mut input_string).expect("Invalid input");
    let input_size: i32 = input_string.trim().parse()
                                        .expect("Invalid number entered");
    println!("Enter the elements one by one");
    for _ in 0..input_size {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Invalid input");
        let num_int: i32 = num.trim().parse().expect("Invalid number");
        list.push(num_int);
    }

    list
}

fn find_median(mut input: Vec<i32>) ->f64 {
    input.sort();
    let mut ans: f64 = -1.0;
    let size = input.len();
    if size%2 == 0 {
       if let Some(num1) = input.get(size/2) {
            if let Some (num2) = input.get(size/2 -1 ) {
                ans = ((*num1) as f64 + (*num2) as f64)/2.0;
            }   
       } 
    }
    else {
        if let Some(num) = input.get((size - 1) / 2) {
            ans = (*num).into();
        }
    }
    ans
}

fn find_mode(input: Vec<i32>) -> i32 {
    let mut hsh = HashMap::new();
    for i in input {
        let val = hsh.get(&i);
        if let Some(value) = val {
            hsh.insert(i, value + 1);
        }
        else {
            hsh.insert(i, 1);
        }
    }
    let mut max = -1;
    let mut ans= -1;
    for (key, value) in hsh.iter() {
        if *value > max {
            max = *value;
            ans = *key; 
        } 
    }
    ans
}
