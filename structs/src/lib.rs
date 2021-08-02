pub struct User{
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}
pub fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
//Tuple Struct
pub struct Color(pub i32, pub i32, pub i32);
pub struct Point(pub i32, pub i32, pub i32);

//类单元结构体unit-like structs
pub struct Coopy;