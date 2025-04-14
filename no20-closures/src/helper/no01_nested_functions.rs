#![allow(unused)]

pub fn main() {
    println!("=== no01_nested_functions ===");
    let multiplier = 3;

    fn multiply_by(value: i32) -> i32 {
        // Invalid line
        // value * multiplier
        2
    }

    println!("{}", multiply_by(5));
}
