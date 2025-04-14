pub fn main() {
    println!("=== no03_non_lexical_lifetimes ===");
    // lexical = lasting until the end of the block
    // non-lexical = not lasting until the end of the block

    let dog = String::from("Milo");
    let my_pet = &dog;
    println!("{my_pet};") // my_pet lifetime end here (last use of reference)
    // 100 lines of code
    // ...
    // ...
}
