#![allow(unused)]

pub fn main() {
    println!("=== no04_invalid_lifetimes ===");
    let some_cities = {
        let cities = vec![
            String::from("London"),
            String::from("New York"),
            String::from("Barcelona"),
        ];
        // Invalid line (return ref out of the block)
        // &cities[..2]
    };

    {
        let cities = vec![
            String::from("London"),
            String::from("New York"),
            String::from("Barcelona"),
        ]; // cities lifetime start

        let favorite_cities = &cities[0..2]; // favorite_cities lifetime start
        println!("{favorite_cities:?}");
        let places = cities; // favorite_cities and cities lifetimes end
        // println!("{favorite_cities:?}"); // dangling ref
        // println!("{cities:?}");
    }
}
