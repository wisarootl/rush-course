#![allow(unused)]

#[derive(PartialEq, Eq)]
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

pub fn main() {
    println!("=== no16_implementing_the_eq_trait ===");

    // eq trait is sub-trait of partialeq with additional properties below
    // - reflexive: a == a
    // symmetric: a == b implies b == a
    // transitive: a ==b and b == c implies a == c

    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "23:20");
    let c = Flight::new("New York", "Los Angeles", "08:00");
    let d = Flight::new("New York", "London", "08:00");
    println!("{}", a == a);
    println!("{}", a == b);
    println!("{}", b == a);
    println!("{}", b == c);
    println!("{}", a == d);

    println!("===");

    let division = 0.0 / 0.0;
    println!("{}", division);

    let value = 3.4;
    println!("{}", value == value);
    println!("{}", division == division);
}
