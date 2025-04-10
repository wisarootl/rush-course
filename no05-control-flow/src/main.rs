fn main() {
    if_statement();
    else_if_statement();
    else_statement();
    assigning_result_of_if_statement_to_variable();
    match_statement();
    underscore_in_a_match_arm();
    match_statement_with_multiple_values_and_conditionals();
    the_loop();
    while_loop();
    recursion();
}

fn if_statement() {
    println!("=== if_statement ===");

    let some_condition_that_we_cannot_predict_in_advance = true;

    if some_condition_that_we_cannot_predict_in_advance {
        println!("This line will be output");
    }

    if false {
        println!("This line will NOT be output");
    }
}

fn else_if_statement() {
    println!("=== else_if_statement ===");
    let season = "summer";

    if season == "summer" {
        println!("School's out!");
    } else if season == "winter" {
        println!("Brr, so cold!");
    } else if season == "fall" {
        println!("Leaves falling!")
    } else if season == "spring" {
        println!("Lots of rain!")
    }

    if season == "summer" {}

    if season == "winter" {}
}

fn else_statement() {
    println!("=== else_statement ===");
    let season = "spring";

    if season == "summer" {
        println!("School's out!");
    } else if season == "winter" {
        println!("Brr, so cold!");
    } else {
        println!("Lots of rain!");
    }
}

fn assigning_result_of_if_statement_to_variable() {
    println!("=== assigning_result_of_if_statement_to_variable ===");
    _even_or_odd(17);
    _even_or_odd(100);
}

fn _even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}");
}

fn match_statement() {
    println!("=== match_statement ===");
    let evaluation = true;

    match evaluation {
        true => {
            println!("The value is true");
        }
        false => {
            println!("the value is false");
        }
    }

    let value = match evaluation {
        true => 20,
        false => 40,
    };

    println!("{value}");
}

fn underscore_in_a_match_arm() {
    println!("=== underscore_in_a_match_arm ===");
    let season = " ";

    match season {
        "summer" => println!("School's out!"),
        "winter" => println!("Brr, so cold!"),
        _ => println!("Lots of rain!"),
    }
}

fn match_statement_with_multiple_values_and_conditionals() {
    println!("=== match_statement_with_multiple_values_and_conditionals ===");
    let number = 8;

    match number {
        2 | 4 | 6 | 8 => println!("{number} is even."),
        1 | 3 | 5 | 7 => println!("{number} is odd."),
        _ => unreachable!(),
    }

    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        value if value % 2 != 0 => println!("{value} is an odd number"),
        _ => unreachable!(),
    }
}

fn the_loop() {
    println!("=== the_loop_and_break_keywords ===");
    let mut seconds = 21;

    loop {
        if seconds <= 0 {
            println!("Blastoff! 🚀");
            break;
        }

        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds...");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }
}

fn while_loop() {
    println!("=== while_loop ===");
    let mut seconds = 21;

    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds...");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }

    println!("Blastoff! 🚀");
}

fn recursion() {
    _countdown(5);
}

fn _countdown(seconds: i32) {
    if seconds == 0 {
        println!("Blastoff!")
    } else {
        println!("{seconds} seconds to blastoff...");
        _countdown(seconds - 1);
    }
}
