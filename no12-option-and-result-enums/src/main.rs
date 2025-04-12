fn main() {
    the_option_enum();
    real_example_of_option_enum();
    unwrap_and_except_methods();
    match_keyword_with_option_enum();
    returning_an_option_enum_from_a_function();
    top_level_option_variants();
    the_unwrap_or_method();
    building_option_from_scratch();
    the_result_enum();
    real_example_of_result_enum();
    returning_a_result_enum_from_a_function();
    nuances_of_unwrap_method();
    the_while_let_construct();
    project();
}

#[allow(unused_variables)]
fn the_option_enum() {
    // option enum equivalent to None in Python
    // `Option::None` absent value
    // `Option::Some(T)` present value
    println!("=== the_option_enum ===");

    let a = Option::Some(5);
    let b = Option::Some("hello");
    let c = Option::Some(true);

    let a: Option<i8> = Option::Some(5);
    let a = Option::<i16>::Some(5);

    let d: Option<bool> = Option::None;
}

fn real_example_of_option_enum() {
    println!("=== real_example_of_option_enum ===");
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instruments.get(2);
    println!("{:?}", bass);

    let invalid_instrument = musical_instruments.get(100);
    println!("{:?}", invalid_instrument);
}

fn unwrap_and_except_methods() {
    println!("=== unwrap_and_except_methods ===");
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];
    println!("- valid");
    let bass: Option<&String> = musical_instruments.get(2);
    println!("{:?}", bass); // Some("Bass")
    let valid_instrument = bass.unwrap();
    println!("{valid_instrument}"); // Bass

    // expect is unwrap with custom message
    let valid_instrument = bass.expect("Unable to retrieve element");
    println!("{valid_instrument}"); // Bass

    println!("- invalid");
    let invalid_instrument = musical_instruments.get(100);
    println!("{:?}", invalid_instrument); // None

    // unwrap and expect will lead to runtime error for invalid value
    // invalid_instrument.unwrap(); // thread 'main' panicked, called `Option::unwrap()` on a `None` value

    // invalid_instrument.expect("Unable to retrieve musical instrument"); // thread 'main' panicked, Unable to retrieve musical instrument
}

fn match_keyword_with_option_enum() {
    println!("=== match_keyword_with_option_enum ===");
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass = musical_instruments.get(2);
    _play(bass);
    println!("{:?}", bass); // Some("Bass")

    let invalid_instrument = musical_instruments.get(100);
    _play(invalid_instrument);
}

fn _play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }
}

fn returning_an_option_enum_from_a_function() {
    println!("=== returning_an_option_enum_from_a_function ===");
    fn _is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
        if item_is_in_system && item_is_in_stock {
            Option::Some(true)
        } else if item_is_in_system {
            Option::Some(false)
        } else {
            Option::None
        }
    }

    let availability = _is_item_in_stock(true, false);

    match availability {
        // Option::Some(value) => println!("Item is available: {value}"),
        Option::Some(true) => println!("Yes, the item is available"),
        Option::Some(false) => println!("No, the item is not in stock"),
        Option::None => println!("Your item doesn't exist in our system"),
    }
}

fn top_level_option_variants() {
    println!("=== top_level_option_variants ===");

    // simplify `Option::Some` to `Some` as they are Rust prelude
    // Rust prelude is a collection of named constructs that are available automatically in every program
    fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
        if item_is_in_system && item_is_in_stock {
            Some(true)
        } else if item_is_in_system {
            Some(false)
        } else {
            None
        }
    }
    let availability = is_item_in_stock(true, false);

    match availability {
        Some(true) => println!("Yes, the item is available"),
        Some(false) => println!("No, the item is not in stock"),
        None => println!("Your item doesn't exist in our system"),
    }
}

fn the_unwrap_or_method() {
    println!("=== the_unwrap_or_method ===");
    // unwrap with default value. (fallback if None)

    let present_value = Some(13);
    let missing_value: Option<i32> = None;
    let missing_value_bool: Option<bool> = None;

    println!("{}", present_value.unwrap_or(0));
    println!("{}", missing_value.unwrap_or(0));
    println!("{}", missing_value_bool.unwrap_or(true));
}

#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh oh"),
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

fn building_option_from_scratch() {
    println!("=== building_option_from_scratch ===");
    let some_option = MyOption::Some(100);
    println!("{}", some_option.unwrap());
    println!("{}", some_option.unwrap_or(13));

    let none_option = MyOption::None;
    println!("{}", none_option.unwrap_or(13));
    // println!("{}", none_option.unwrap()); // panicked with Uh oh
}

fn the_result_enum() {
    println!("=== the_result_enum ===");
    let ok: Result<i8, &str> = Result::Ok(5); // i8 is for ok, &str is for err
    println!("{:?}", ok);
    let ok: Result<i8, &str> = Ok(5);
    println!("{:?}", ok);
    let disaster: Result<i32, &str> = Err("Something went wrong"); // Err is like exception in Python
    println!("{:?}", disaster);
}

fn real_example_of_result_enum() {
    println!("=== real_example_of_result_enum ===");
    let text = "50";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number); // Ok(50)

    let text = "Alabama";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number); // Err(ParseIntError { kind: InvalidDigit })

    let text = "05";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number); // Ok(5)
}

fn _divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}
fn _handle_result(result: Result<f64, String>) {
    println!("_handle_result()");
    match result {
        Ok(calculation) => println!("Result: {}", calculation),
        Err(message) => println!("Error: {}", message),
    }
}
fn _handle_result2(result: Result<f64, String>) {
    println!("_handle_result2()");
    println!("{}", result.is_ok());
    println!("{}", result.is_err());
}

fn returning_a_result_enum_from_a_function() {
    println!("=== returning_a_result_enum_from_a_function ===");
    println!("- valid");
    let result = _divide(10.0, 2.0);
    _handle_result(result.clone());
    _handle_result2(result.clone());
    println!("{}", result.clone().unwrap());
    println!("{}", result.clone().expect("Unable to parse calculation"));
    println!("{}", result.clone().unwrap_or(0.0));

    println!("- invalid");
    let result = _divide(10.0, 0.0);
    _handle_result(result.clone());
    _handle_result2(result.clone());
    // println!("{}", result.clone().unwrap()); // panic: called `Result::unwrap()` on an `Err` value: "Cannot divide by zero"
    // println!("{}", result.clone().expect("Unable to parse calculation")); // panic Unable to parse calculation: "Cannot divide by zero"
    println!("{}", result.clone().unwrap_or(0.0)); // 0
}

fn _operation(great_success: bool) -> Result<&'static str, &'static str> {
    if great_success {
        Ok("Success")
    } else {
        Err("Error")
    }
}

fn _operation2(great_success: bool) -> Result<String, String> {
    if great_success {
        Ok("Success".to_string())
    } else {
        Err("Error".to_string())
    }
}

fn nuances_of_unwrap_method() {
    println!("=== nuances_of_unwrap_method ===");
    // no ownership move example =====
    let my_result = _operation(true);

    let content = match my_result {
        Ok(message) => message,
        Err(error) => error,
    }; // no ownership move since there is copy trait for &str

    println!("{}", my_result.unwrap());
    println!("{}", my_result.unwrap());
    println!("{}", my_result.unwrap());
    println!("{}", content);

    // ownership move example ====
    let my_result = _operation2(true);
    let content = match my_result {
        Ok(message) => message,
        Err(error) => error,
    };
    // println!("{}", my_result.unwrap()); // Error
    // my_result result is invalid here since ownership move to content
    // because there is no copy trait for String
    println!("{}", content);
}

fn the_while_let_construct() {
    println!("=== the_while_let_construct ===");
    let mut sauces = vec!["Mayonaise", "Ketchup", "Ranch"];

    while let Some(sauce) = sauces.pop() {
        println!("The next sauce is {sauce}");
    }
}

fn project() {
    /*
    Define a Food struct with a single `name` field
    set to a String. Derive a Debug implementation.

    Define a Restaurant struct with a `reservations` field
    set to a u32 and a `has_mice_infestation` field set to
    a bool. Derive a Debug implementation.

    Define a `chef_special` method on the Restaurant.
    The method will return the restaurant's famous
    dish. It should return an Option containing a Food
    struct.

    If the restaurant has a mice infestation, return the
    None variant. There is no chef special!

    If the restaurant has less than 12 reservations, return
    a Food instance with a name of "Uni Sashimi" wrapped in
    the Some variant. If it has 12 or more reservations,
    return a Food instance with a name of "Strip Steak"
    instead, also wrapped in the Some variant.

    Define a `deliver_burger` method on the Restaurant.
    It should accept an `address` string slice; it will
    represent where to deliver the order. It should
    return a Result type where the Ok variant holds a Food
    struct and the Err variant holds a String.

    If the restaurant has a mice infestation, return the
    Err variant containing a String of "Sorry, we have a
    mice problem".

    If the user's address is an empty string, return the Err
    variant with a String of "No delivery address specified".
    HINT: You can use the `is_empty` method on a string to check
    if it has 0 characters.
    https://doc.rust-lang.org/std/string/struct.String.html#method.is_empty

    Otherwise, the delivery is good to go! Return the Ok
    variant containing a Food struct with a `name` of "Burger".

    In the `main` function, create a `Restaurant` instance
    with 11 reservations and a mice infestation.

    Invoke the `chef_special` method and print out its return
    value. It should be the None variant.

    Invoke the `deliver_burger` method with an argument of "123
    Elm Street" and print out its return value. It should be
    the Err variant.

    Create another `Restaurant` instance with 15 reservations
    and no mice infestation.

    Invoke the `chef_special` method and print out its return
     value. It should be the Some variant with a "Strip Steak".

    Invoke the `deliver_burger` method with an argument of an
    empty address. Print out its return value. It should be the
    Err variant.

    Invoke the `deliver_burger` method again with an argument
    of a valid address. Print out its return value. It should
    be the Ok variant nesting a Food struct with a `name` of
    "Burger".
    */
    println!("=== project ===");

    let marios = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    println!("{:?}", marios.chef_special());
    println!("{:?}", marios.deliver_burger("123 Elm Street"));

    let angelos = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };
    println!("{:?}", angelos.chef_special());
    println!("{:?}", angelos.deliver_burger(""));
    println!("{:?}", angelos.deliver_burger("123 Elm Street"));
}

#[allow(dead_code)]
#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        }

        if self.reservations < 12 {
            Some(Food {
                name: String::from("Uni Sashimi"),
            })
        } else {
            Some(Food {
                name: String::from("Strip Steak"),
            })
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err(String::from("Sorry, we have a mice problem"));
        }

        if address.is_empty() {
            return Err(String::from("No delivery address specified"));
        }

        Ok(Food {
            name: String::from("Burger"),
        })
    }
}
