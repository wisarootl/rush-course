use std::io;

pub fn main() {
    println!("=== no05_collecting_user_input_with_read_line_method ===");

    let mut name = String::new();
    println!("What is your name?");

    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to collect input from the user");

    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}", name.trim()),
        Err(message) => println!("There was an error: {message}"),
    }
}
