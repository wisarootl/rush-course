pub fn main() {
    println!("=== no01_review_of_strings ===");
    // &str: string slice
    // String: heap string, mutable

    let pirate = "Bloodhook"; // string literal
    let filepath = r"C:\My Documents\new\videos"; // raw string
    println!("{filepath}");

    let sailer = String::from(pirate); // create heap string from string literal

    let bad_guy = sailer.to_string(); // similar to `String::from(pirate)`

    println!("{pirate} and {sailer} and {bad_guy}");

    let first_char = &pirate[0..1];
    println!("{first_char}");

    let first_initial = &pirate[0..4];
    println!("{first_initial}");
}
