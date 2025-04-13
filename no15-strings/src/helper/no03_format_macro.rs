pub fn main() {
    println!("=== no03_concatenation ===");

    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    let icon = format!("{0} {1} {0}", first_name, last_name); // as thing as println!
    println!("{icon}");
    println!("{first_name}");
    println!("{last_name}");
}
