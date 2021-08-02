use vectors::SpreedsheetCell;
fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3, 4];
    let sheets = vec![
        SpreedsheetCell::Int(5),
        SpreedsheetCell::Float(10.1),
        SpreedsheetCell::Text(String::from("Test spreedcell")),
    ];

    println!("This is v2_first {}", &v2[0]);

    v1.push(100);
    v1.push(101);
    v1.push(102);
    v1.push(103);
    println!("This is v1_third {}", &v1[2]);

    match v1.get(4) {
        Some(four) => println!("This is v1_four {}", four),
        None => println!("No value"),
    }

    for i in &mut v2 {
        println!("v2 are {}", &i);
        *i = *i + 1;
        println!("v2 + 1 are {}", &i);
    }
    println!("Hello, world!");
}
