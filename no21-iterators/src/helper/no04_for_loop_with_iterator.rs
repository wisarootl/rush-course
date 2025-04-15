pub fn main() {
    println!("=== no04_for_loop_with_iterator ===");

    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.into_iter();

    for number in my_iterator {
        println!("{number}");
    }

    // println!("{my_vector} {my_iterator}");
    // my_vector lost ownership to my_iterator
    // my_iterator lost ownership to for-loop

    let my_vector = vec![4, 8, 15, 16, 23, 42];
    for number in my_vector {
        println!("{number}");
    }
}
