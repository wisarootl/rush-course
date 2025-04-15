pub fn main() {
    println!("=== no05_why_iterator_can_be_immutable ===");

    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.into_iter();

    for number in my_iterator {
        println!("{number}");
    }
}
