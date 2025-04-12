fn main() {
    // Ownership
    scope_and_ownership();
    copy_trait();
    string_type();
    push_str_method_on_a_string_type();
    moves_and_ownership();
    drop_function();
    clone_method();
    references_and_borrowing();
    string_literal();
    copy_trait_with_references();
    ownership_and_function_parameters();
    mutable_parameters();
    return_values();
    project();
}

fn scope_and_ownership() {
    println!("=== scope_and_ownership ===");
    let age = 33;
    {
        let is_handsome = true;
        println!("{is_handsome}");
    } // is_handsome goes out of scope here
    println!("{age}");
    // println!("{is_handsome}");

    // age variable exists here
} // age goes out of scope, then age variable goes out of scope here

fn copy_trait() {
    println!("=== copy_trait ===");
    // copy_trait mean type can be copied.
    let time = 2025;
    let years = time;

    println!("The time is {time}. It is the year {years}.");
}

fn string_type() {
    println!("=== string_type ===");
    let food: &str = "pasta"; // static string (known size at compile time), store in section of the binary
    let text: String = String::new(); // dynamic string, store on the heap
    let candy = String::from("KitKat");

    dbg!(food);
    dbg!(text);
    dbg!(candy);
}

fn push_str_method_on_a_string_type() {
    println!("=== push_str_method_on_a_string_type ===");
    let mut name = String::from("Boris");
    println!("{name}");

    name.push_str(" Pask");
    println!("{name}");

    name.push_str("haver");
    println!("{name}");
}

fn moves_and_ownership() {
    println!("=== moves_and_ownership ===");
    let person = String::from("Boris");
    println!("My name is {person}");

    let genius = person; // owner is moved from person to _genius

    // println!("My name is {person}"); // error[E0382]: borrow of moved value: `person`
    println!("My name is {genius}");
}

fn drop_function() {
    let person = String::from("Boris");

    drop(person); // rust implicitly call drop at the end of blocks for all local variable.
    // println!("My name is {person}"); // error[E0382]: borrow of moved value: `person`

    // let genius = person;
}

fn clone_method() {
    println!("=== clone_method ===");
    let person = String::from("Boris");
    let genius = person.clone(); // no ownership move, we copied the string.

    println!("This is {person}.");
    println!("This is {genius}.");
}

fn references_and_borrowing() {
    // use value without moving ownership
    println!("=== references_and_borrowing ===");

    // ampersand (&) is called "borrow operator"
    // & can read "address leading to"

    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value;

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;

    dbg!(my_integer_reference);
    dbg!(my_heap_reference);

    // reference must never outlive their referent.

    // dereference
    println!("{}", *my_integer_reference);
    println!("{}", *my_heap_reference);

    // rust implement display trait for references.
    println!("{}", my_integer_reference);
    println!("{}", my_heap_reference);
}

fn string_literal() {
    /*
      String - A dynamic piece of text stored on the heap
      at runtime

      &String ("ref String") - A reference to a heap String

      str - A hardcoded, read-only piece of text encoded in
      the binary

      &str ("ref str") - A reference to th text in the memory
      that has loaded the binary file
    */
    println!("=== string_literal ===");

    let ice_cream = "Cookies and Cream";
    println!("{}", ice_cream);

    /*
        - String for changing, mutating, updating text content (dynamic)
        - &String prevent duplicate copy of String
        - &str known content in advance (compile time)
    */
}

fn copy_trait_with_references() {
    println!("=== copy_trait_with_references ===");
    let ice_cream = "Cookies and Cream";
    let dessert = ice_cream; // no ownership movement. it is copied.
    println!("{ice_cream} {dessert}.");
}

fn ownership_and_function_parameters() {
    println!("=== ownership_and_function_parameters ===");
    let oranges = String::from("Oranges");
    _print_my_value(oranges); // let value = oranges; no copy here, ownership move.
    // println!("{oranges} is now invalid");

    let apple: i32 = 6;
    _print_my_value_i32(apple); // apple is copied to value. no ownership moved.
    dbg!(apple);
}

fn _print_my_value(value: String) {
    println!("Your value is {value}");
}

fn _print_my_value_i32(value: i32) {
    println!("Your value is {value}");
}

fn mutable_parameters() {
    println!("=== mutable_parameters ===");
    let burger = String::from("Burger");
    _add_fries(burger);
}

fn _add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}");
}

fn return_values() {
    println!("=== return_values ===");
    let cake = _bake_cake();
    println!("I now have a {cake} cake!");

    let mut current_meal = String::new();
    current_meal = _add_flour(current_meal);
    println!("{current_meal}");
}

fn _bake_cake() -> String {
    // cake
    String::from("Chocolate Mousse")
}

fn _add_flour(mut meal: String) -> String {
    meal.push_str("Add flour");
    meal
}

fn project() {
    /*
    Declare a `is_concert` variable set to a boolean.
    Declare a `is_event` variable assigned to `is_concert`.
    Will Rust move ownership? State your answer, then confirm
    by trying to printing both variables out.

    Declare a `sushi` variable to set to a string literal of "Salmon"
    Declare a `dinner` variable assigned to the `sushi` variable.
    Will Rust move ownership? State your answer, then confirm
    by trying to printing both variables out.

    Repeat the previous example but use a heap String instead.
    Will Rust move ownership? Explain why the result is different
    from the previous operation.

    The `clear` method modifies a heap String to have no content.
    Declare an `eat_meal` function that accepts a `meal` parameter
    of type String. In the body of `eat_meal`, invoke the `clear`
    method on the `meal` parameter.

    In the `main` function, invoke the `eat_meal` function and pass
    in your "Salmon" String. Explain what happens when the eat_meal
    function runs. Describe the complete movement of ownership of
    the "Salmon" String throughout the program.

    Say we want to keep the String around after `eat_meal` is
    called. How can we continue to have access to the String in
    the `main` function? Print out the (empty) String.
    */
    println!("=== project ===");

    let is_concert = true;
    let is_event = is_concert; // no ownership move
    println!("{is_concert} {is_event}");

    let sushi = "Salmon";
    let dinner = sushi; // no ownership move
    println!("{sushi} {dinner}");

    let sushi = String::from("Salmon");
    let dinner = sushi; // ownership move
    // println!("{dinner} {sushi}");

    let fish = _eat_meal(dinner); // ownership move
    dbg!(fish);
}

fn _eat_meal(mut meal: String) -> String {
    meal.clear();
    meal
}
