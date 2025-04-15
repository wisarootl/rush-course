pub fn main() {
    println!("=== no06_iter_method ===");
    // iter method is iterator of immutable references

    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.iter();
    for number in my_iterator {
        print!("{number} ");
    }
    println!("\n===");

    for number in &my_vector {
        // implicit .iter from ref &my_vector
        print!("{number} ");
    }
    println!("\n===");

    let cities = vec![String::from("Phoenix"), String::from("Dallas")];

    for city in cities.iter() {
        println!("{city}");
    }

    for city in &cities {
        println!("{city}");
    }
    println!("{cities:?}");

    for city in cities {
        println!("{city}");
    } // cities lost ownership to for-loop here
}
