use std::collections::HashMap;

pub fn main() {
    println!("=== no01_create_a_hashmap_with_new_function ===");
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert(String::from("Steak"), 29.99);
    menu.insert(String::from("Tuna"), 29.29);
    menu.insert(String::from("Burger"), 14.99);

    println!("{:?}", menu);

    // let mut country_capitals: HashMap<&str, &str> = HashMap::new();
    let mut country_capitals = HashMap::<&str, &str>::new(); // turbo fish
    // let mut country_capitals = HashMap::new();

    country_capitals.insert("France", "Paris");
    country_capitals.insert("Germany", "Berlin");

    println!("{:?}", country_capitals);
}
