#![allow(unused)]

// #[derive(PartialEq)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

pub fn main() {
    println!("=== no13_implementing_the_partial_eq_trait_for_structs ===");
    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "23:20");
    let c = Flight::new("New York", "Los Angeles", "08:00");

    println!("{}", a == b);
    println!("{}", a.eq(&b));
    println!("{}", a.ne(&b));
    println!("{}", a != b);
    println!("{}", a == c);

    // let division = 0.0 / 0.0;
    // println!("{}", division);

    // let value = 3.4;
    // println!("{}", value == value);
    // println!("{}", division == division);
}
