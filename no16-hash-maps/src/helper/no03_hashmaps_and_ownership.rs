use std::collections::HashMap;

pub fn main() {
    println!("=== no03_hashmaps_and_ownership ===");

    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");
    println!("{}", coffee_pairings.len());
    println!("{drink} {milk}");
}
