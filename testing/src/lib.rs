#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    pub value: u32,
}
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("The value must bigger than 0, but value is {}", value);
        } else if value > 100 {
            panic!("The value must smaller than 100, but value is {}", value);
        }
        Guess{
            value,
        }
    }
}

#[cfg(test)]
mod tests;
