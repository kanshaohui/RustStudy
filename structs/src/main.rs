use structs::{User, build_user};
use structs::Color;
use structs::Point;
fn main() {
    let mut user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    let user2 = build_user(String::from("user2name@example.com"), String::from("usernameuser2"));
    let user3 = User {
        email: String::from("thirdemail@example.com"),
        username: String::from("thirdname"),
        ..user2
    };
    //tuple struct
    //tuple struct 整体有个名，内部元素没有名字
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Hello, world!");
}
