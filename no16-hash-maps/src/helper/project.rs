use std::collections::HashMap;

pub fn main() {
    /*
    Bring the HashMap type into the current's file's namespace.

    Declare a `sauces_to_meals` HashMap. The keys will be
    string slices and the values will be a vector of string
    slices. Use the `from` function to populate the HashMap
    with 2 key-value pairs:

    Key: "Ketchup"
    Value: Vector of ["French Fries", "Burgers", "Hot Dogs"]

    Key: "Mayonnaise"
    Value: Vector of ["Sandwiches", "Burgers", "Coleslaw"]

    Use the `insert` method to add the following key-value
    pair to the HashMap.

    Key: "Mustard"
    Value: Vector of ["Hot dog", "Burgers", "Pretzels"]

    Use the `remove` method to remove the key-value pair
    where "Mayonnaise" is the key. Find a way to retrieve
    the vector inside the Option and print it out.

    Use the `get` method to retrieve the key-value pair
    where "Mustard" is the key. Find a way to retrieve
    the vector inside the Option and print it out.

    Use the `entry` and `or_insert` methods to add the
    following key-value pair:

    Key: "Soy Sauce"
    Value: Vector of ["Sushi", "Dumplings"]

    Finally, print out the final `sauces_to_meals` HashMap.

    The final result should be:
    {
      "Ketchup": ["French Fries", "Burgers", "Hot Dogs"],
      "Soy Sauce": ["Sushi", "Dumplings"],
      "Mustard": ["Hot dog", "Burgers", "Pretzels"]
    }
    */
    println!("=== project ===");

    let mut sauces_to_meals = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);

    println!("{:?}", sauces_to_meals.remove("Mayonnaise").unwrap());

    let mustard_meals = sauces_to_meals.get("Mustard");
    match mustard_meals {
        Some(meals) => println!("The meals were {meals:?}"),
        None => println!("There were no meals for that sauce! Oh no"),
    }

    sauces_to_meals
        .entry("Ketchup")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!("{sauces_to_meals:?}");
}
