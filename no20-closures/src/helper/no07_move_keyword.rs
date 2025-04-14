pub fn main() {
    println!("=== no07_move_keyword ===");

    let first_name = String::from("Alice");
    let last_name = String::from("Wonder");
    let capture_string = move || {
        println!("{first_name} {last_name}");
    }; // move = move ownership
    // println!("{first_name} {last_name}"); // first_name and last_name is not valid after this block
    capture_string();
    capture_string();
    capture_string();
}
