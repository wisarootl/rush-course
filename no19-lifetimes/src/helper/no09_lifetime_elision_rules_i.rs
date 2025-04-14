#![allow(unused)]

/*
Elision rules are as follows: (Inferred to to omit lifetime annotation)

- The compiler assigns a lifetime to each parameter that is a reference.

- If there is one reference parameter and the return value is a reference,
  the borrow checker will infer that their lifetimes are related.

- If there are multiple reference parameters but one of them is self (instance),
  the borrow checker will assume the lifetime of the instance (self) is connected
  to the lifetime of the return value

Otherwise, it is an error to elide an output lifetime.
*/

// first and second rules
fn my_awesome_function(first: &i32, second: String) -> &i32 {
    first
}

// first and second rules
fn select_first_two_elements(items: &[String]) -> &[String] {
    &items[0..2]
}

pub fn main() {
    println!("=== no09_lifetime_elision_rules_i ===");

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
