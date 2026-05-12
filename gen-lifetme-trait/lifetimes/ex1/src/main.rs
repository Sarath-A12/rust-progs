fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("Value of r is {}", r);
}
