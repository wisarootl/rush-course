fn main() {
    integers();
    using_underscore();
    usize_isize();
    strings_and_raw_strings();
    methods();
    floats();
    casting_types();
    math_operations();
    augmented_assignment_operator();
    booleans();
    equality_and_inequality_operators();
    logics_operators();
    character_types();
    array_types();
    display_traits();
    debug_traits();
    dbg_macro();
    tuple_types();
    range_types();
    generics();
    project();
}

fn integers() {
    // println!("=== integers ===");
    // let eight_bit: i8 = -210; // error: literal out of range for `i8`
    // let eight_bit: u8 = -1; // error[E0600]: cannot apply unary operator `-` to type `u8`

    let _sixteen_bit_signed: i16 = -32500;
    let _sixteen_bit_unsigned: u16 = 64000;

    let _thirty_two_bit_signed: i32 = -2147483648;
    let _thirty_two_bit_unsigned: u32 = 4294967295;

    let _some_value = 20u16; // define 20 with type u16
}

fn using_underscore() {
    let _sixteen_bit_signed: i16 = 32_500;
    let _sixteen_bit_signed: i16 = -3_2_500;
    let _sixteen_bit_signed: i16 = -3_2_5_00;
}

fn usize_isize() {
    // usize
    // isize
    let _days: usize = 55;
    let _years: isize = -15_000;

    // the size is depending on computer architecture
    // 32 for 32 bits system and 64 for 64 bits system
}

fn strings_and_raw_strings() {
    println!("=== strings_and_raw_strings ===");
    // literal strings
    println!("Dear Emily,\nHow have you been?");
    println!("\tOnce upon a time");
    println!("Juliet said \"I love you Romeo\"");

    // raw strings
    // let filepath = "C:\My Documents\new\videos";
    let filepath = "C:\\My Documents\\new\\videos";
    println!("{filepath}");
    let filepath = r"C:\My Documents\new\videos";
    println!("{filepath}");
}

fn methods() {
    println!("=== methods ===");
    let val: i32 = -15;
    println!("{}", val.abs());

    let empty_space = "     my content    ";
    println!("{}", empty_space.trim());

    println!("{}", val.pow(2));
    println!("{}", val.pow(3));
}

fn floats() {
    println!("=== float ===");
    let pi: f64 = 3.1415926535897932384;
    println!("The current value of pi is {pi}");
    println!("The current value of pi is {:.4}", pi);
    println!("{}", pi.floor());
    println!("{}", pi.ceil());
    println!("{}", pi.round());
}

fn casting_types() {
    println!("=== casting_types ===");
    let miles_away = 50;
    let _miles_away_i8 = miles_away as i8;
    let _miles_away_u8: u8 = miles_away as u8;

    let miles_away: f64 = 100.329032;
    let _miles_away_f32 = miles_away as f32;
    let miles_away_int = miles_away as i32;
    println!("{miles_away_int}");
}

fn math_operations() {
    println!("=== math_operations ===");
    // + is called operator
    // 5 and 4 is operands
    let addition = 5 + 4;
    let subtraction = 10 - 6;
    let multiplication = 3 * 4;
    println!("Addition: {addition}, subtraction: {subtraction}, multiplication: {multiplication}");

    let floor_division = 5 / 3;
    println!("{floor_division}");

    let decimal_division = 5.0 / 3.0;
    println!("{decimal_division}");

    let remainder = 9 % 2;
    println!("{remainder}");
}

fn augmented_assignment_operator() {
    println!("=== augmented_assignment_operator ===");
    let mut year = 2025;
    year += 1;
    println!("The new year is {year}");

    year -= 5;
    println!("The new year is {year}");

    year *= 2;
    println!("The new year is {year}");

    year /= 4;
    println!("The new year is {year}");
}

fn booleans() {
    println!("=== booleans ===");
    let is_handsome: bool = true;
    let is_silly: bool = false;

    println!("Handsome: {is_handsome}. Silly: {is_silly}");

    let age: i32 = -40;
    let is_young = age < 35;
    println!("{is_young}");
    println!("{} {}", age.is_positive(), age.is_negative());

    // boolean inversion
    println!("{}", !true);
    println!("{}", !false);

    let age = 13;
    let can_see_rated_r_movie = age >= 17;
    let cannot_see_rated_r_movie = !can_see_rated_r_movie;
    println!("I am {age} years old. Can I not see this scary movie? {cannot_see_rated_r_movie}");
}

fn equality_and_inequality_operators() {
    println!("=== equality_and_inequality_operators ===");
    println!("{}", "Coke" == "Pepsi");
    println!("{}", "Coke" != "Pepsi");
    println!("{}", "Coke" == "coke");
    println!("{}", "Coke" == "Coke ");
    println!("{}", "Coke" == "Coke");

    println!("{}", 13 == 13);
    println!("{}", 13 != 13);

    println!("{}", 26.1 == 26.1);
    println!("{}", 26.1 == 26.14);

    println!("{}", 13 == 13.1 as i32);

    println!("{}", true == true);
    println!("{}", false == false);
    println!("{}", true != false);
}

fn logics_operators() {
    println!("=== logics_operators ===");
    let purchased_ticket = true;
    let plane_on_time = true;
    let making_event = purchased_ticket && plane_on_time;
    println!("It is {} that I will arrive as expected", making_event);

    let user_has_paid_for_subscription = false;
    let user_is_admin = false;
    let user_can_see_premium_experience = user_has_paid_for_subscription || user_is_admin;
    println!("Can this user see my site? {user_can_see_premium_experience}");
}

fn character_types() {
    println!("=== character_types ===");
    let first_initial = 'B';
    let emoji = '🎧';

    println!("{} {}", first_initial, emoji);

    println!(
        "{} {}",
        first_initial.is_alphabetic(),
        emoji.is_alphabetic()
    );

    println!("{} {}", first_initial.is_uppercase(), emoji.is_uppercase());
    println!("{} {}", first_initial.is_lowercase(), emoji.is_lowercase());
}

fn array_types() {
    println!("=== array_types ===");
    // - compound types: consist of may value
    // - scalar: only one value
    // array is homogeneous (same type for all values)
    let _numbers: [i32; 6] = [4, 8, 15, 16, 23, 42];

    let apples = ["Granny Smith", "McIntosh", "Red Delicious"];
    println!("Length: {}", apples.len());

    let _currency_rates: [f64; 0] = [];

    let mut seasons = ["Spring", "Summer", "Fall", "Winter"];

    println!("{}", seasons[2]);
    seasons[2] = "Autumn";
    println!("{}", seasons[2]);
}

fn display_traits() {
    println!("=== display_traits ===");
    // display trait can be represented as string
    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);

    // arrays is not display trait
    let _seasons = ["Spring", "Summer", "Fall", "Winter"];
}

fn debug_traits() {
    println!("=== debug_traits ===");

    // debug trait can be represented as string for debugging purpose.
    let seasons = ["Spring", "Summer", "Fall", "Winter"];

    println!("{:?}", 5);
    println!("{0:?}", 3.14);
    println!("{:?}", true);
    println!("{seasons:?}"); // debug traits
    println!("{seasons:#?}"); // pretty print debug traits
}

fn dbg_macro() {
    // debug macro
    println!("=== dbg_macro ===");
    let seasons = ["Spring", "Summer", "Fall", "Winter"];

    dbg!(2 + 2);
    println!("{seasons:#?}");

    dbg!(seasons);
}

fn tuple_types() {
    // tuple is compound types and heterogeneous (values are not the same type)
    println!("=== tuple_types ===");
    let employee = ("Molly", 32, "Marketing");

    let name = employee.0;
    let age = employee.1;
    let department = employee.2;
    println!("Name: {name}, age: {age}, department: {department}");

    let (name, age, department) = employee;
    println!("Name: {name}, age: {age}, department: {department}");
}

fn range_types() {
    println!("=== range_types ===");
    let month_days = 1..31; // upper value is exclusive
    println!("{month_days:?}");

    let month_days = 1..=31; // upper value is inclusive
    println!("{month_days:?}");

    for number in month_days {
        print!("{number} ");
    }
    println!();

    let letters = 'a'..'f';

    for letter in letters {
        println!("{letter}");
    }

    let colors = ["Red", "Green", "Yellow"];

    for color in colors {
        println!("{color} is a great color!");
    }
}

fn generics() {
    // a generic is a type argument
    let _month_days: std::ops::Range<i32> = 1..31;
    let _letters: std::ops::Range<char> = 'b'..'f';

    // Value - 5
    // Type - i32

    // std means rust standard library
    // :: is navigating child directory
}

fn project() {
    /*
    Declare an i32 variable assigned to 1337.
    Use the underscore character to add a visual
    separator between the numbers.

    Cast the i32 to an i16 integer and assign the result
    to a separate variable.

    Declare a floating-point value of your choosing.
    Print out the number with 3 digits of precision.

    Declare a 'with_milk' variable set to a Boolean.
    Declare a 'with_sugar` variable set to a Boolean.

    Declare a 'is_my_type_of_coffee` variable. It should
    be set to true if the coffee has both milk and sugar.

    Declare an `is_acceptable_coffee` variable. It should
    be set to true if the coffee has either milk or
    sugar.

    Declare an array with four i8 integers of your choosing
    Print out the array in its Debug representation.

    Declare a tuple consisting of the integer, float,
    a Boolean, and the array that you previously declared.
    Print out the tuple in its Debug representation.
    */
    println!("=== project ===");

    let distance: i32 = 1_337;
    let miles = distance as i16;

    let height = 150.34546;
    println!("{height:.3}");

    let with_milk: bool = true;
    let with_sugar: bool = true;
    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;

    let distances: [i32; 4] = [13, 23, 75, 100];
    println!("{distances:#?}");

    let combo = (
        miles,
        height,
        is_my_type_of_coffee,
        is_acceptable_coffee,
        distances,
    );
    println!("{combo:#?}")
}
