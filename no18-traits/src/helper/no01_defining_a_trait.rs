#![allow(unused)]

use std::{collections::HashMap, fmt::Display};

trait Accommodation {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
    fn book(&mut self, name: &str, nights: u32);
}

trait Description {
    fn get_full_description(&self) -> String {
        String::from("A full description")
    }
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl Description for Hotel {}

#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnB {
    fn get_full_description(&self) -> String {
        format!("A full description. Please enjoy {}'s apartment", self.host)
    }
}

fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    entity.book(guest, 1);
}

// trait bound
// `<T: Accommodation>` is same as `impl Accommodation`
fn book_for_two_nights<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 2);
}

// fn book_two_places<T: Accommodation>(first: T, second: T, guest: &str) {
// ...
// }
// fn book_two_places<T: Accommodation, U: Accommodation>(first: T, second: U, guest: &str) {
// ...
// }
fn book_two_places(first: &mut impl Accommodation, second: &mut impl Accommodation, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 1);
}

// `<T: Accommodation + Description>` = `(impl Accommodation + Description)`
fn book_two_places_and_describe_first(
    first: &mut (impl Accommodation + Description),
    second: &mut impl Accommodation,
    guest: &str,
) {
    first.book(guest, 1);
    first.get_full_description();

    second.book(guest, 1);
}

fn book_two_places_and_describe_first_2<T: Accommodation + Description, U: Accommodation>(
    first: &mut T,
    second: &mut U,
    guest: &str,
) {
    book_two_places_and_describe_first(first, second, guest)
}

// where clauses
fn book_two_places_and_describe_first_3<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    book_two_places_and_describe_first(first, second, guest)
}

fn choose_best_place_to_stay() -> impl Accommodation + Description {
    Hotel::new("The Luxe")
}

#[derive(Debug)]
struct Hotel2<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

impl<T> Hotel2<T> {
    fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Description for Hotel2<T> {
    fn get_full_description(&self) -> String {
        format!("Hotel2: {} is the pinnacle of luxury", self.name)
    }
}

pub fn main() {
    println!("=== no01_defining_a_trait ===");

    let mut hotel = Hotel::new("The Luxe");
    book_for_one_night(&mut hotel, "Piers");
    book_for_two_nights(&mut hotel, "A");
    println!("{}", hotel.get_description());
    println!("{}", hotel.summarize());
    hotel.book("Piers", 5);
    println!("{:#?}", hotel);

    let mut airbnb = AirBnB::new("Peter");
    book_for_one_night(&mut airbnb, "Amanda");
    book_for_two_nights(&mut airbnb, "B");
    println!("{}", airbnb.get_description());
    airbnb.book("Piers", 3);
    println!("{:#?}", airbnb);

    book_two_places(&mut hotel, &mut airbnb, "Piers");
    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);

    book_two_places_and_describe_first(&mut hotel, &mut airbnb, "John");
    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);

    book_two_places_and_describe_first_2(&mut hotel, &mut airbnb, "John");
    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);

    book_two_places_and_describe_first_3(&mut hotel, &mut airbnb, "John");
    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);

    let mut hotel2 = choose_best_place_to_stay();
    println!("{}", hotel2.get_description());

    let hotel3 = Hotel2::new(String::from("The Luxe"));
    println!("{}", hotel3.get_full_description());
}
