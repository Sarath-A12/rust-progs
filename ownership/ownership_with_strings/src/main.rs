fn main() {
    // Possible because s is in heap
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x;
    // this is fine and x is still valid

    let s1 = String::from("Hello");
    let s2 = s1;

    // fine, but s1 won't be valid

    // println!("{}, {}", s1, s2);
    let mut s = String::from("Hello");
    s = String::from("Ahoy");
    println!("{}, world", s);

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);
}
