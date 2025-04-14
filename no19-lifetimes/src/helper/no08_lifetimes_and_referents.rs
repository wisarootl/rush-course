fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
}

pub fn main() {
    println!("=== no08_lifetimes_and_referents ===");

    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let two_cities = {
        let cities_ref = &cities;
        select_first_two_elements(cities_ref)
    };
    println!("{two_cities:?}");

    {
        let coffees = [String::from("Latte"), String::from("Mocha")];
        let two_coffees = select_first_two_elements(&coffees);
        println!("{two_coffees:?}");
    }
}
