use traits::lodging::{Accommodation, AirBnB, Description, Hotel};
use traits::utils;

pub fn main() {
    println!("=== no04_moving_to_project_structure ===");
    let mut hotel = Hotel::new(String::from("The Luxe"));
    println!("{}", hotel.get_description());
    hotel.book("Dena", 5);

    let mut airbnb = AirBnB::new("Parker");
    println!("{}", airbnb.get_description());
    utils::book_for_one_night(&mut airbnb, "Dan");

    utils::mix_and_match(&mut hotel, &mut airbnb, "Phil");
}
