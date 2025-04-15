// use std::collections::HashSet;

pub fn main() {
    println!("=== no12_map_method_i ===");

    let numbers = vec![4, 8, 15, 16, 23, 42];

    let squares: Vec<i32> = numbers.iter().map(|number: &i32| number.pow(2)).collect();

    // turbo fish approach
    // let squares = numbers
    //     .iter()
    //     .map(|number: &i32| number.pow(2))
    //     .collect::<HashSet<i32>>();

    println!("{squares:?}");
    println!("{numbers:?}");
}
