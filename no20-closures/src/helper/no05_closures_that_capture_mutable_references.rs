pub fn main() {
    println!("=== no05_closures_that_capture_mutable_references ===");
    let mut numbers = vec![4, 8, 15, 16, 23, 42];
    let mut add_number = || numbers.push(100);
    add_number();
    add_number();
    add_number(); // numbers lifetime end here.
    println!("{numbers:?}");
}
