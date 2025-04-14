fn select_first_two_elements(items: &[String]) -> &[String] {
    &items[..2]
}

pub fn main() {
    println!("=== no06_references_as_function_parameters ===");

    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let two_cities = select_first_two_elements(&cities);
    println!("{two_cities:?}");

    {
        let coffees = [String::from("Latte"), String::from("Mocha")];
        let two_coffees = select_first_two_elements(&coffees);
        println!("{two_coffees:?}");
    }
}
