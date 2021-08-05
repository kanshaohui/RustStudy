use std::fmt::Display;

pub fn largest_num(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &i in list.iter() {
        if largest < i {
            largest = i;
        }
    }
    largest
}
pub fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_all<T: Copy + PartialOrd> (list: &[T]) -> T {
    let mut result = list[0];

    for &item in list.iter() {
        if item > result{
            result = item;
        }
    }

    result
}

pub fn longest<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

pub fn longest_with_annoucement <'a, T> (x: &'a i32, y: &'a i32, ann: T) -> &'a i32
    where T: Display
{
    println!("Annoucement for longest is {}", ann);
    if x > y {
        x
    } else {
        y
    }
}