fn main() {
    // Reference and borrowing
    immutable_and_mutable_reference_parameters();
    multiple_immutable_references();
    mutable_reference_restrictions();
    ownership_with_immutable_and_mutable_references();
    dangling_references();
    ownership_with_arrays_and_tuples();
    project();
}

fn immutable_and_mutable_reference_parameters() {
    /*
    options for parameter
    `meal: String`: take full ownership of the string. have no permission to modify
    `mut meal: String`: take full ownership of the string. have permission to modify
    `meal: &String`: reference. no ownership of the string. have no permission to modify
    `meal: &mut String`: reference. no ownership of the string. have permission to modify
    */
    println!("=== immutable_and_mutable_reference_parameters ===");
    let mut current_meal = String::new();
    _add_flour_reference(&mut current_meal);
    _show_my_meal(&current_meal);
}

fn _add_flour_reference(meal: &mut String) {
    meal.push_str("Add flour");
}

fn _show_my_meal(meal: &String) {
    println!("Meal steps: {meal}");
}

fn multiple_immutable_references() {
    println!("=== multiple_immutable_references ===");
    // value can have any number of immutable references
    let car = String::from("Red");
    let ref1 = &car;
    let ref2 = &car;
    println!("{ref1} and {ref2} and {}", &car);
}

fn mutable_reference_restrictions() {
    println!("=== mutable_reference_restrictions ===");
    // value can only have a single mutable reference at at time
    let mut car = String::from("Red");
    let ref1 = &mut car;
    println!("{ref1}");
    let ref2 = &mut car;
    // println!("{ref1}"); // ref1 invalid here
    println!("{ref2}");
}

fn ownership_with_immutable_and_mutable_references() {
    println!("=== ownership_with_immutable_and_mutable_references ===");
    let mut coffee = String::from("Mocha");

    let a = &coffee;
    let b = a; // same as let b = &coffee;
    println!("{a} {b}");

    // ====
    let a = &mut coffee;
    println!("{a}");
    let b = a; // move mut ref from a to b. a is invalid now
    println!("{b}");
}

fn dangling_references() {
    println!("=== dangling_references ===");
    let city = _create_city();
    println!("{city}");
}

/*
fn _create_city_ref() -> &String {
    let city_ref = String::from("New York");
    &city_ref // in valid ref return (dangling_references)
    /*
    cannot return reference to local variable `city_ref`
    returns a reference to data owned by the current function
    */
}
*/

fn _create_city() -> String {
    String::from("New York")
}

fn ownership_with_arrays_and_tuples() {
    // any compound is always the owner on any nested value.
    println!("=== ownership_with_arrays_and_tuples ===");
    let registrations = (true, false, true);
    let first = registrations.0; // value is copied
    println!("{first} and {registrations:?}");

    let languages = (String::from("Rust"), String::from("JavaScript"));
    let first = &languages.0;
    println!("{first} and {languages:?}");
}

fn project() {
    /*
    Let's model a road trip!

    Define a `start_trip` function that creates and returns
    a String of "The plan is..."

    Invoke the `start_trip` function in `main` and save its
    return value to a `trip` variable.

    We want to pass the String to three separate functions
    that will mutate the String without transferring ownership.

    Define a `visit_philadelphia` function that concatenates
    the text "Philadephia" to the end of the String. Invoke
    the function in `main`. Then, invoke `push_str` on the String
    to concatenate the content " and " to the end. Mak sure to
    include the spaces.

    Define a `visit_new_york` function that concatenates the
    text "New York" to the end of the String. Invoke the function
    in `main`. Repeat the previous logic to concatenate " and "
    to the end of the String.

    Define a `visit_boston` function that concatenates the
    text "Boston." to the end of the String. Invoke the function
    in `main`. Concatenate a period to the end of the
    String/sentence.

    Define a `show_itinerary` function that will print out
    the final version of the String. Find a way to do so
    without transferring ownership.

    Invoke `show_itinerary`. The final output should be:

    "The plan is...Philadelphia and New York and Boston."
    */
    println!("=== project ===");

    let mut trip = _start_trip();
    _visit_philadelphia(&mut trip);
    trip.push_str(" and ");
    _visit_new_york(&mut trip);
    trip.push_str(" and ");
    _visit_boston(&mut trip);
    trip.push('.');
    _show_itinerary(&trip);
}

fn _start_trip() -> String {
    String::from("The plan is...")
}

fn _visit_philadelphia(trip: &mut String) {
    trip.push_str("Philadelphia");
}

fn _visit_new_york(trip: &mut String) {
    trip.push_str("New York");
}

fn _visit_boston(trip: &mut String) {
    trip.push_str("Boston");
}

fn _show_itinerary(trip: &String) {
    println!("{trip}");
}
