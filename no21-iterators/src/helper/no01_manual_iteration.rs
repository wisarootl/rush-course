pub fn main() {
    println!("=== no01_manual_iteration ===");
    let numbers = vec![4, 8, 15, 16, 23, 42];

    // manual loop
    let mut current_index = 0;
    let final_index = numbers.len() - 1;

    loop {
        if current_index > final_index {
            break;
        }

        print!("{} ", numbers[current_index]);
        current_index += 1;
    }
    println!("\n===");

    // while loop
    let mut current_index = 0;
    let final_index = numbers.len() - 1;

    while current_index <= final_index {
        print!("{} ", numbers[current_index]);
        current_index += 1;
    }
    println!("\n===");

    // for loop
    for number in numbers {
        print!("{number} ");
    }
    println!("\n");
}
