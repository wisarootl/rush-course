fn main() {
    basic_variables();
    immutable_mutable_variables();
    variable_shadowing();
    scopes();
    constant();
    type_aliases();
    compiler_directive();
    coding_challenge();
}

fn basic_variables() {
    println!("=== basic_variables ===");
    let apples = 50;
    let oranges = 14 + 6;
    let _fruits = apples + oranges;

    println!("This year, my garden has {apples} apples.");
    println!(
        "This year, my garden has {} apples and {}.",
        apples, oranges
    );
    println!(
        "This year, my garden has {0} apples and {1}. I can't believe I have {0} apples.",
        apples, oranges
    );
}

fn immutable_mutable_variables() {
    println!("=== immutable_mutable_variables ===");
    let mut gym_reps = 10;
    println!("I plan to do {gym_reps} reps");

    gym_reps = 15;
    println!("I now plan to do {gym_reps} reps");
}

fn variable_shadowing() {
    println!("=== variable_shadowing ===");
    let grams_of_protein = "100.345";
    println!("{grams_of_protein}");
    let grams_of_protein = 100.345;
    println!("{grams_of_protein}");
    let mut grams_of_protein = 100;
    println!("{grams_of_protein}");
    grams_of_protein = 105;
    println!("{grams_of_protein}");
}

fn scopes() {
    println!("=== scope ===");
    let coffee_price = 5.99;

    println!("1. The coffee price is {coffee_price}");
    {
        println!("2. The coffee price is {coffee_price}");
        let coffee_price = 1.99;
        println!("3. The coffee price is {coffee_price}");
    }

    println!("4. The coffee price is {coffee_price}");
}

const TAX_RATE: f64 = 7.25;

fn constant() {
    println!("=== constant ===");
    let income: i32 = 100000;
    println!("My income is {income} and my tax rate is {TAX_RATE}");
    const TAX_RATE_2: f64 = 8.25;
    println!("My income is {income} and my tax rate is {TAX_RATE_2}");
}

type Meters = i32;

fn type_aliases() {
    println!("=== constant ===");
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
    println!(
        "A one mile race is {mile_race_length} meters long and a two mile race is {two_mile_race_length} meters long."
    );
}

// #[allow(unused_variables)] // all in the next function
fn compiler_directive() {
    // #![allow(unused_variables)]
    #[allow(unused_variables)] // all next line only
    let mile_race_length: Meters = 1600;
    // let two_mile_race_length: Meters = 3200;

    {
        #![allow(unused_variables)]
        // - put this at the top of file to allow the whole file
        let mile_race_length: Meters = 1600;
        let two_mile_race_length: Meters = 3200;
    }

    // #[allow(unused_variables)] for line/function
    // #![allow(unused_variables)] for file and block
}

const TOUCHDOWN_POINTS: i32 = 6;

fn coding_challenge() {
    /*
    Declare a `season` variable set to a string with
    your favorite season. Provide an explicit type annotation.
    The type of a string is a `&str`. We'll discuss what
    the & symbol means later in the course.

    Declare a `points_scored` variable set to 28.
    Provide an explicit type annotation. The type of
    an integer is `i32`.

    It's time to update the team's score. Declare the
    `points_scored` variable to be mutable. Set its
    new value to 35.

    Declare a `TOUCHDOWN_POINTS` constant at the file
    level set to the value 6.

    Declare a `event_time` variable set to a string of
    "06:00".

    Use variable shadowing to redeclare `event_time` set
    to a integer of 6.

    Use interpolation to print out all of the
    declared variables and constants in a println! call.
    Practice with direct interpolation, sequential
    arguments, and numeric arguments.

    Declare a `favorite_beverage` variable set to a string
    of your favorite drink. Use an underscore to silence
    the compiler warning about the variable being unused.

    Remove the underscore. Provide a compiler directive
    to silence the compiler warning about the variable
    being unused.
    */

    println!("=== coding_challenge ===");
    let season: &str = "Fall";
    #[allow(unused_assignments)]
    let mut points_scored: i32 = 28;
    points_scored = 35;

    #[allow(unused_variables)]
    let event_time: &str = "06:00";
    let event_time: i32 = 6;

    println!(
        "My favorite season is {season}. The team scored {points_scored} points. The event started at {event_time}. A touchdown is wroth {TOUCHDOWN_POINTS} points."
    );
    println!(
        "My favorite season is {}. The team scored {} points. The event started at {}. A touchdown is wroth {} points.",
        season, points_scored, event_time, TOUCHDOWN_POINTS
    );
    println!(
        "My favorite season is {0}. The team scored {1} points. The event started at {2}. A touchdown is wroth {3} points.",
        season, points_scored, event_time, TOUCHDOWN_POINTS
    );

    #[allow(unused_variables)]
    let favorite_beverage = "Snapple Apple";
}
