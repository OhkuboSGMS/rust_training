use std::collections::HashMap;

pub mod front_of_house;
pub mod back_of_house;

pub use crate::front_of_house::hosting;

fn serve_order() {}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("i'd like {} toast please", meal.toast);
    // crate::front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();
    // meal.seasonal_fruis = String::from("blueberries")

    let order1 = back_of_house::Appetizer::Salad;
    hosting::add_to_waitlist();
}