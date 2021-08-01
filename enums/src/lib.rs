pub enum IpAddrKind{
    V4,
    V6,
}

pub struct IpAddr{
    pub kind: IpAddrKind,
    pub address: String,
}

pub enum IpAddrN {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub enum Message{
    Quit,
    Move{x: i32, y: i32,},
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
pub enum UsState{
    Alabama,
    Alaska,
}

impl Message {
    pub fn call(&self){

    }
}

impl IpAddrN {
    pub fn router(iptype: IpAddrKind) {

    }
}

impl Coin {
    pub fn value(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State is {:?}", state);
                25
            },
        }
    }
}