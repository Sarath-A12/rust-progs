/*
*Using a hash map and vectors, create a text interface to allow a user to add
employee names to a department in a company; for example, “Add Sally to Engineering” or
“Add Amir to Sales.” Then, let the user retrieve a list of all people in a department or all
people in the company by department, sorted alphabetically.
*/
use std::collections::HashMap;
use std::io::stdin;
fn main() {
    println!("Enter the query. Enter Ctrl + C to exit");
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        query_processor(&mut departments);
    }
}

fn query_processor(dict: &mut HashMap<String, Vec<String>>) {
    let mut query = String::new();
    stdin().read_line(&mut query).expect("Error occured");

    let mut parts = query.split_whitespace();
    let command = parts.next().expect("Invalid command entered");
    if command.eq_ignore_ascii_case("Add") {
        add_query(&query, dict);
    } else if command.eq_ignore_ascii_case("Get") {
        get_query(&query, dict);
    }
}

fn get_query(query: &String, dict: &mut HashMap<String, Vec<String>>) {
    for (num, word) in query.split_whitespace().enumerate() {
        if num == 0 {
            continue;
        }
        if num == 1 && word.eq_ignore_ascii_case("company") {
            let mut vec: Vec<String> = dict.keys().cloned().collect();
            vec.sort();
            for department in vec {
                println!("The members of department {}", department);
                get_department_sorted(&department, dict);
            }
        }
        if num == 2 {
            get_department_sorted(word, dict);
        }
    }
}

fn get_department_sorted(word: &str, dict: &HashMap<String, Vec<String>>) {
    if let Some(values) = dict.get(word) {
        let mut temp = values.clone();
        temp.sort();
        for value in temp {
            println!("{}", value);
        }
    } else {
        println!("Department does not exist!");
    }
}

fn add_query(query: &str, dict: &mut HashMap<String, Vec<String>>) {
    let mut name = String::new();
    for (num, word) in query.split_whitespace().enumerate() {
        if num == 0 || num == 2 {
            continue;
        }
        if num == 1 {
            name = word.to_string();
        }
        if num == 3 {
            if let Some(vect) = dict.get_mut(word) {
                vect.push(name.to_string());
            } else {
                dict.insert(word.to_string(), vec![name.clone()]);
            }
        }
    }
}
