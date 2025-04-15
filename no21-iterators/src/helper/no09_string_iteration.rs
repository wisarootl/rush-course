pub fn main() {
    println!("=== no09_string_iteration ===");

    let seafood = String::from("Oyster🦪");

    for byte in seafood.bytes() {
        print!("{byte}/");
    }

    println!();

    for character in seafood.chars() {
        print!("{character}/");
    }
    println!();

    println!("{seafood}");

    println!("{}", seafood.bytes().len()); // 10
    println!("{}", seafood.chars().count()); // 7
}
