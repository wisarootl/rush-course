pub fn main() {
    println!("=== no10_string_retain_method ===");
    let mut game_console = String::from("PlayStation");
    let mut deleted_characters = String::new();

    let closure = |character| {
        let is_not_a = character != 'a';
        if is_not_a {
            true
        } else {
            deleted_characters.push(character);
            false
        }
    };
    game_console.retain(closure);
    println!("{game_console}");
    println!("{deleted_characters}");
}
