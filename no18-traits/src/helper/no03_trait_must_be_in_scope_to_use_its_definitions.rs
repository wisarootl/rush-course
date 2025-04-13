use std::ops::Add;
use std::str::FromStr;

pub fn main() {
    println!("=== no03_trait_must_be_in_scope_to_use_its_definitions ===");
    // explicit use trait for execute binary code optimization
    let a: i32 = 5;
    let b: i32 = 10;
    let sum = a.add(b);
    println!("{sum}");

    let numeric_count = u64::from_str("5");
    println!("{}", numeric_count.unwrap());
}
