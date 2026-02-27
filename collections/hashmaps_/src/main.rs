use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 15);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // copied : Opn&i32 -> Opn32
    println!("{score}");

    for (key, value) in &scores {
        println!("{key} - {value}");
    }

    // Won't work because scores already is being used in the above loop
    // for (key, value) in scores {
    //     println!("{key} - {value}");
    // }
    // Use &scores otherwise

    //ints get copied into hashmap - Strings don't
    

    // Three ways to deal with duplicates
    
    // One - insert to overwrite 
    scores.insert(String::from("Blue"), 13);

    // Two - check if it exists, if it does ignore. Else write
    scores.entry(String::from("Blue")).or_insert(25);
    scores.entry(String::from("Green")).or_insert(55);

    println!("{scores:?}");


    // Third - updating based on old value - a counter. Implemented using entry. insert()

}
