fn largest(number_list: &[i32]) -> &i32 {
    let mut largest = &number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number in list 1 is {}", result);
    // println!("The largest number is {}", largest);

    let number_list = vec![134, 150, 333, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number in list 1 is {}", result);
}
