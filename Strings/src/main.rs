fn main() {
    let mut s = String::new();
    s = "Initial content".to_string();
    let hello = String::from("你好!");

    let mut s_add = String::from("foo");
    let s_added = "bar";
    s_add.push_str(s_added);
    println!("s_add {} add s_added {}", s_add, s_added);

    let s3 = s + &hello;
    println!("s3 {} = s add {} ", s3, hello);

    let s1 = String::from("tic");
    let s2 = String::from("toc");
    let s3 = String::from("toe");
    let s =s1 + "-" + &s2 + "-" + &s3;
    println!("use + {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("toc");
    let s3 = String::from("toe");
    let s = format!("{} - {} - {}", s1, s2, s3);
    println!("{} + {} + {}", s1, s2, s3);
    println!("use format! {}", s);

    let length = hello.len();
    let s_add_len = s_add.len();
    println!("hello length is {}, s_add length is {}", length, s_add_len);

    for c in hello.chars() {
        println!("{}", c);
    }

    for c in hello.bytes() {
        println!("{}", c);
    }

    println!("Hello, world!");
}
