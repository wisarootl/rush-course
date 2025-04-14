// Invalid
// fn create_slice(items: Vec<i32>) -> &[i32] {
//     &items
//     &items[0..2]
//     &items[0]
// }

// Invalid
// fn create_number_reference(number: i32) -> &i32 {
//     &number
// }

pub fn main() {
    println!("=== no05_functions_cannot_return_references_to_owned_values_or_parameters ===");
    // Invalid
    // create_number_reference(10)
}
