use std::collections::HashMap;

pub fn main() {
    println!("=== no04_access_a_value_by_key ===");

    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");

    let value = coffee_pairings["Latte"]; // get error if no key
    println!("{value}");

    // let value = coffee_pairings["Latte1"]; // error no entry found for key

    let value = coffee_pairings
        .get("Flat White")
        .copied()
        .unwrap_or("Unknown Milk");

    println!("{value}");

    let value = coffee_pairings
        .get("Flat White1")
        .copied()
        .unwrap_or("Unknown Milk");

    println!("{value}");
}
