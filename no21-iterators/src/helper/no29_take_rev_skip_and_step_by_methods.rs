pub fn main() {
    println!("=== no29_take_rev_skip_and_step_by_methods ===");
    let fifty_numbers = 1..=50;

    // take is like df.head
    // skip: skip head
    for value in fifty_numbers.clone().take(15).skip(5).step_by(2) {
        print!("{value}/")
    }

    println!();
    println!("{fifty_numbers:?}");
    // rev --> reverse
    for value in fifty_numbers.clone().rev().take(15).skip(5).step_by(2) {
        print!("{value}/")
    }

    println!();
    println!("{fifty_numbers:?}");
}
