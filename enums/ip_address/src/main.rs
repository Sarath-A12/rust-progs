enum IpAddrKind {
    V4, 
    V6,
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}
// Better way to put data
enum IpAddr {
    V4(String), // Can also put V4(u8, u8, u8, u8) -> Better than struct
    V6(String),
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(&four);
    route(&six);
    // How to store data using enums

    /*
    One Way
     */
    let home = IpAddrStruct {
        kind: four,
        address: String::from("127.0.0.1"),
    };

    let loopback= IpAddrStruct {
        kind: six,
        address: String::from("::1"),
    };
    /*
    Enum inside struct method
    */

    //Another way
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
    // Constructor function
}

fn route (ip_kind: &IpAddrKind) {
    //Function useful because it's valid
    // for any of IpAddrKind
}