use std::collections::HashMap;
fn main() {
    let mut h = HashMap::new();
    h.insert(String::from("Blue"), 10);
    h.insert(String::from("Yellow"), 20);
    println!("{:#?}", h);

    let firstvalue = vec![String::from("Red"), String::from("Green")];
    let secondvalue = vec![30, 40];
    let a: HashMap<_, _> = firstvalue.iter().zip(secondvalue).collect();
    println!("{:#?}", a);

    for (k, v) in &a {
        println!("a.{} => a.{}", k, v);
    }

    //h.insert(String::from("Green"), 50);
    h.entry(String::from("Green")).or_insert(51);
    //h.insert(String::from("Red"), 60);
    h.entry(String::from("Red")).or_insert(61);
    h.insert(String::from("Blue"), 70);
    h.entry(String::from("Blue")).or_insert(71);
    for (k, v) in &h {
        println!("h.{} => h.{}", k, v);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    for (k, v) in &map {
        println!("word:{} => {}", k, v);
    }
}
