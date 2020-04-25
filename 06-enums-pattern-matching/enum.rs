#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// we can mix structs & enums like sum & product types in Haskell
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// enum variants can also hold values
#[derive(Debug)]
enum Ip {
    V4(String),
    V6(String),
}

enum Ip2 {
    V4(u8, u8, u8, u8), // each value of IPv4 is between 0 & 255
    V6(String)
} // having different types for each variant is not possible with struct

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // this variant has anonymous struct as data
    Write(String),
    ChangeColor(i32, i32, i32),
}

// defining methods on enum

impl Message {
    fn call(&self) {
        // method body
    }
}

// handle null values - included in prelude
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("Home is {:?}", home);

    println!("Concise home is {:?}", Ip::V4(String::from("127.0.0.1")));
    println!("Concise loopback is {:?}", Ip::V6(String::from("::1")));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");
    let absent_number: Option<i32> = Option::None; // For None, we need to tell rust type of T
    // We are using enum defined by us hence Option::
    // stdlib has thees types that can be used directly
    println!("some_number = {:?}", some_number);
    println!("some_string = {:?}", some_string);
    println!("absent_number = {:?}", absent_number);
}

// both V4 & V6 are of same type: IpAddrKind
fn route(ip_kind: IpAddrKind) {
    println!("ip_kind = {:?}", ip_kind);
}
