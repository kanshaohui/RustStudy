fn yanghui_triangle(max: i32) {
    let mut yang: Vec<i32> = Vec::new();
    yang = [1].to_vec();
    println!("{:?}", yang);
    yang.push(1);
    println!("{:?}", yang);
    let mut n = 2;
    while n < max {
        let mut a = 1;
        let temp_yang = yang.clone();
        yang = [1, 1].to_vec();
        while a < n {
            yang.insert(a as usize, temp_yang[(a - 1) as usize] + temp_yang[a as usize]);
            a = a + 1;
        }
        println!("{:?}", yang);
        n = n + 1;
    }
}

fn main() {
    let str=String::from("Yiibai"); 
    yanghui_triangle(10);
    println!("Hello, world! {}", str);
}
