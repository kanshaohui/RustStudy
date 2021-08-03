use std::fs::File;
use std::io::Read;
use std::io::{ErrorKind, Error};
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind(){
                ErrorKind::NotFound => match File::create("hello.txt"){
                    Ok(fc) => fc,
                    Err(error_other) => panic!("创建文件失败:{}", error_other),
                },
                other => panic!("出现其他错误！{:?}", other),
        },
    };
// 另一种写法，采用匿名函数
    let f1 = File::open("none.txt").unwrap_or_else(|error| {
       if error.kind() == ErrorKind::NotFound {
           File::create("none.txt").unwrap_or_else(|error| {
               panic!("创建文件失败：{:?}", error);
           })
       }
       else {
           panic!("出现其他错误！{:?}", error);
       }
    });

    let f3 = File::open("none.txt").unwrap();
//    let f3 = File::open("unwrap.txt").expect("文件没有找到！");

    println!("Hello, world!");
    match read_file_match("hello.txt") {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("文件不存在或者读文件错误！{:?}", e),
    };

    match read_file_question_mark("hello.txt") {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("文件不存在或者读文件错误！{:?}", e),
    }
}

fn read_file_match(s: &str) -> Result<String, Error> {
    let f = File::open(s);
    let  mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}

fn read_file_question_mark(file_name: &str) -> Result<String, Error> {
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}