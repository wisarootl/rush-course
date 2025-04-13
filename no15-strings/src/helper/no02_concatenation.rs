pub fn main() {
    println!("=== no02_concatenation ===");
    // ==== example 1
    let mut full_name = String::from("Sylvester");
    let last_name = "Stallone";

    full_name.push(' ');
    full_name.push_str(last_name);
    println!("{full_name}");

    // ==== example 2
    let mut full_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    full_name.push(' ');
    full_name.push_str(&last_name); // &String -> str
    println!("{full_name}");

    // ==== example 3
    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");
    let full_name = first_name + " " + &last_name; // syntactic sugar
    println!("{full_name}");

    // Invalid
    // println!("{first_name}");
}
