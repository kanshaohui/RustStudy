use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref};
use crate::StackOverList::StackOverCons;

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub struct MyBox<T>(T);
impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn hello(x: &str) {
    println!("欢迎{}", x);
}

pub struct CustomSmartPoint {
    pub data: String,
}
impl Drop for CustomSmartPoint {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPoint Data: {}", self.data);
    }
}

pub enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

#[derive(Debug)]
pub enum RefCellList {
    RefCellCons(Rc<RefCell<i32>>, Rc<RefCellList>),
    RefCellNil,
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl <'a, T> LimitTracker<'a, T>
    where T: Messenger
{
    fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker{
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max > 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max > 0.9 {
            self.messenger.send("Urgent Warning: you've used up over 90% quota!");
        } else if percentage_of_max > 0.75 {
            self.messenger.send("Warning: you've used up over 75% quota!");
        }
    }
}

#[derive(Debug)]
pub enum StackOverList {
    StackOverCons(i32, RefCell<Rc<StackOverList>>),
    StackOverNil,
}
impl StackOverList {
    pub fn tail(&self) -> Option<&RefCell<Rc<StackOverList>>> {
        match self {
            StackOverCons(_, item) => Some(item),
            StackOverNil => None,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub v: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        send_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.send_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn send_an_over_seventy_five_percent_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
    }
}