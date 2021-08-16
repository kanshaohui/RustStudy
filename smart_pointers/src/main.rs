use smart_pointers::{List, MyBox, hello, CustomSmartPoint};
use smart_pointers::List::{Cons, Nil};
use smart_pointers::RcList::{RcCons, RcNil};
use std::rc::Rc;
use std::cell::RefCell;
use smart_pointers::RefCellList::{RefCellCons, RefCellNil};
use smart_pointers::StackOverList::{StackOverCons, StackOverNil};

fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let cons = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Cons(4,
                                                              Box::new(Nil))))))));

    let my_x = 6;
    let my_y = MyBox::new(my_x);
    assert_eq!(6, my_x);
    assert_eq!(6, *my_y);

    let hello_name = MyBox::new(String::from("Hero"));
    hello(&hello_name);

    let sp1 = CustomSmartPoint{
        data: String::from("这是一个测试"),
    };
    let sp2 = CustomSmartPoint {
        data: String::from("这是第二个测试"),
    };

    let rc_cons = Rc::new(RcCons(5,
                                 Rc::new(RcCons(10,
                                                Rc::new(RcNil)))));
    println!("rc_cons's reference count is {}", Rc::strong_count(&rc_cons));
    let rc_cons_left = RcCons(15, Rc::clone(&rc_cons));
    println!("rc_cons's reference count is {} after created rc_cons_left", Rc::strong_count(&rc_cons));
    {
        let rc_cons_right = RcCons(25, Rc::clone(&rc_cons));
        println!("rc_cons's reference count is {} after created rc_cons_right", Rc::strong_count(&rc_cons));
    }
    println!("rc_cons's reference count is {} after released rc_cons_right", Rc::strong_count(&rc_cons));

    let refcell_value = Rc::new(RefCell::new(5));
    let a = Rc::new(RefCellCons(Rc::clone(&refcell_value),
                                Rc::new(RefCellNil)));
    let b = RefCellCons(Rc::new(RefCell::new(33)), Rc::clone(&a));
    let c = RefCellCons(Rc::new(RefCell::new(22)), Rc::clone(&a));
    {
        *refcell_value.borrow_mut() += 10;
    }
    println!("a {:?}", a);
    println!("b {:?}", b);
    println!("c {:?}", c);

    //Stack over test
   let stack_over_a = Rc::new(StackOverCons(5,
                                             RefCell::new(Rc::new(StackOverNil))));
    println!("stack_over_a's count is {}", Rc::strong_count(&stack_over_a));

    let stack_over_b = Rc::new(StackOverCons(10,
                                             RefCell::new(Rc::clone(&stack_over_a))));
    println!("stack_over_a's count is {}", Rc::strong_count(&stack_over_a));
    println!("stack_over_b's count is {}", Rc::strong_count(&stack_over_b));

    if let Some(link) = stack_over_a.tail() {
        *link.borrow_mut() = Rc::clone(&stack_over_b);
    }
    println!("after a->b, stack_over_a's count is {}", Rc::strong_count(&stack_over_a));
    println!("after a->b, stack_over_b's count is {}", Rc::strong_count(&stack_over_b));

    //Stack over
    //println!("a next item {:?}", stack_over_a.tail());
}
