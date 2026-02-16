//Normal struct
struct User {
    active: bool,
    username: String, 
    email: String,
    sign_in_count: u64,
}

//Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit structs ?
struct AlwaysEqual;
fn main() {
    // initialization 
    let mut user1 = User { 
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // entire instance must be mutable

    user1.email = String::from("anotheremail@example.com");

    // let user2 = User { 
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    //alternatively, we can do

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("User 2 is {}", user2.username);
    println!("User 1 email is {}", user1.email);
    // This is fine because email is new string

    // Most of these won't work , because
    // value is moved when user2 = user1.
    //internally string is moved!


    //TUPLE STRUCTS
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Different functions for both.
    // More decoupling
    let Point(x, y, z) = origin;

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // username: username,
        // email: email,
        // Using field init shorthand instead we get
        // works because same name
        username, 
        email,
        sign_in_count: 1,
    }
}
