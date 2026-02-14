fn main() {
    // Crux. How do we ensure that
    // value stays even after function calls
    // REFERENCES
    // REFERENCES ARE GUARANTEED
    // TO POINT TO VALUES THAT ARE
    // VALID VALUE OF PARTICULAR TYPE

    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    let mut s = s1.clone();
    change(&mut s);
    println!("{}", s);

    println!("The length of {s1} is {len}");

    // Only one mutable reference allowed at a time!

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
    // r2 will fail if r1 isn't in it's own scope
    // r1 has the mutable reference
    // atleast until println

    println!("{r2}");


    // ANOTHER ISSUE
    let mut s = s1.clone();

    let r1 = &s;
    let r2 = &s;

    // let r3 = &mut s; 
    // Above line fails!
    // Can't have mutable reference, while
    // we have immutable reference to the same value

    // Main reason is that immutable reference users don't expect the value
    // to change suddenly when they are using it
    // Multiple immutable OK , because no one can change it !



    println!("{r1}, {r2},");
    // However, we can use it now since
    // r1 and r2 have been used, and now
    // they are out of scope
    println!("{r1}, {r2},");
    // Interestingly, reference last till whenever it's used the LAST


    let r3 = &mut s;
    println!("{r3}");
    // Again, the point being that since it's used the last
    // And r1 and r2 won't be used anymore
    // It's okay to use it now
}
// & means references
// Simply put, it means that you refer to the value
// Without taking ownership of the value.

/*
    THIS IS CALLED BORROWING
*/
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
    // Here, s goes out of scope
    // But s doesn't have ownership of it's reference,
    // so the string is not dropped!!
}

fn change(s: &mut String) {
    s.push_str(", world");
    // not allowed for &String
}

/*
DATA RACE--
two or more pointers access same data at same time
atleast one of them is used to write the data
no mechanism to synchronize

This causes undefined behaviours
*/
