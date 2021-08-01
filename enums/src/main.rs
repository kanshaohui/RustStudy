use enums::{IpAddrKind, UsState};
use enums::IpAddr;
use enums::IpAddrN;
use enums::Message;
use enums::Coin;
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: six,
        address: String::from("::1"),
    };
    let home = IpAddrN::V4(127, 0, 0, 1);
    let loopback = IpAddrN::V6(String::from("::1"));

    let m = Message::ChangeColor(0, 0, 0);
    m.call();

    let penny = Coin::Penny;
    println!("{}",penny.value());

    let quater = Coin::Quarter(UsState::Alabama);
    println!("{}", quater.value());

    let some_number = Some(5);
    let none_number: Option<i32> = None;

    if let Some(x) = plus_one(Some(5)) {
        println!("Some(x) x = {}", x);
    }

    println!("Hello, world!");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(z) => Some(z + 1),
        None => None,
    }
}