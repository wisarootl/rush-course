pub fn main() {
    println!("=== no02_concrete_lifetimes_for_references ===");
    // ref do not outlive value
    let dog = String::from("Milo");
    let my_pet = &dog;

    {
        let my_pet = &dog;
        println!("{my_pet}");
    }

    println!("{dog}");

    {
        let my_pet = &dog;
        println!("{my_pet}");
    }
    println!("{my_pet}");
}
