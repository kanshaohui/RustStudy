mod front_of_house;

pub use front_of_house::hosting;
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();

    let order1 = back_of_house::Appetizer::Soap;
    let order2 = back_of_house::Appetizer::Salad;

    let mut m = back_of_house::Breakfast::summer("Rye");
    m.toast = String::from("Wheat");
    println!("I'd like {} toast please!", m.toast);

    hosting::add_to_waitlist();
}

fn serve_order() {

}

mod back_of_house;
