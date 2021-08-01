pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}
impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
pub enum Appetizer {
    Soap,
    Salad,
}

fn fix_incorrect_order() {
    super::serve_order();
    cook_order();
}
fn cook_order() {

}