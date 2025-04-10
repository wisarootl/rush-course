fn main() {
    intro_to_functions();
    return_values();
    unit_as_a_return_type();
    blocks_in_functions();
    project();
}

fn intro_to_functions() {
    println!("=== intro_to_functions ===");
    _open_store("Brooklyn");
    _bake_pizza(20, "pepperoni");
    _swim_in_profit();
    _swim_in_profit();
    _swim_in_profit();
    _open_store("Queens");
    _bake_pizza(15, "mushroom");
}

fn _open_store(neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}");
}

fn _bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizzas");
}

fn _swim_in_profit() {
    println!("So much $$$, so little time");
}

fn return_values() {
    println!("=== return_values ===");
    let result = _square(5);
    println!("The square of 5 is {result}");

    let result = _square(13);
    println!("The square of 13 is {result}");

    let result = _square_implicit(5);
    println!("The square of 5 is {result}");

    let result = _square_implicit(13);
    println!("The square of 13 is {result}");
}

fn _square(number: i32) -> i32 {
    return number * number;
}

fn _square_implicit(number: i32) -> i32 {
    number * number // implicitly return last line without ";""
}

fn unit_as_a_return_type() {
    println!("=== unit_as_a_return_type ===");
    // unit is empty tuple `()`
    // unit is default return values if no explicit or implicit return value
    let _result = mystery();
}

fn mystery() {
    println!("Hello there");
}

fn blocks_in_functions() {
    println!("=== blocks_in_functions ===");
    let multiplier = 3;

    let calculation = {
        let value = 5 + 4;
        value * multiplier
    };

    println!("{calculation}");
}

fn project() {
    /*
    Define an apply_to_jobs function that accepts a
    'number' parameter (an i32) and a 'title' parameter
    (a string). It should print out the string:
    "I'm applying to {number} {title} jobs".

    Example:
    apply_to_jobs(35, "Rust Developer")
    -> "I'm applying to 35 Rust Developer jobs"

    Define an is_even function that accepts a 'number'
    parameter (an i32). The function should return a true
    if the number is even and a false if the number is
    odd.
    Examples:
    is_even(8) -> true
    is_even(9) -> false

    Define an alphabets function that accepts a 'text'
    parameter (an &str). The function should return a
    tuple of two Booleans. The first Boolean should check
    if the text contains the letter 'a'. The second
    Boolean should check if the text contains the letter
    'z'. You can use the 'contains' method to check if a
    string contains a specific character. See the documentation:
    https://doc.rust-lang.org/std/primitive.str.html#method.contains

    Examples:
    println!("{:?}", alphabets("aardvark")); -> (true, false)
    println!("{:?}", alphabets("zoology"));  -> (false, true)
    println!("{:?}", alphabets("zebra"));    -> (true, true)
    */
    println!("=== project ===");

    _apply_to_jobs(35, "Rust Developer");

    println!("{}", _is_even(8));
    println!("{}", _is_even(9));

    println!("{:?}", _alphabets("aardvark"));
    println!("{:?}", _alphabets("zoology"));
    println!("{:?}", _alphabets("zebra"));
}

fn _apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} {title} jobs");
}

fn _is_even(number: i32) -> bool {
    number % 2 == 0
}

fn _alphabets(text: &str) -> (bool, bool) {
    (text.contains("a"), text.contains("z"))
}
