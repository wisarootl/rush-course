// <'a> (tick a) is generic life time

// items input lifetime is related to return value lifetime
fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
}

pub fn main() {
    println!("=== no07_intro_to_generic_lifetimes ===");

    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let two_cities = select_first_two_elements(&cities);
    // drop(cities);
    println!("{two_cities:?}");

    {
        let coffees = [String::from("Latte"), String::from("Mocha")];
        let two_coffees = select_first_two_elements(&coffees);
        println!("{two_coffees:?}");
    }
}
