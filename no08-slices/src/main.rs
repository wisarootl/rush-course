fn main() {
    create_a_string_slice_from_a_string();
    string_slices_and_string_literals();
    string_slice_lengths();
    syntactic_shortcuts();
    string_slices_as_function_parameters();
    array_slices();
    deref_coercion_with_array_slices();
    mutable_array_slices();
    project();
}

fn create_a_string_slice_from_a_string() {
    println!("=== create_a_string_slice_from_a_string ===");
    let action_hero = String::from("Arnold Schwarzenegger");
    let first_name = &action_hero[0..6];
    println!("{first_name}");

    let last_name = &action_hero[7..21];
    println!("{last_name}");
}

fn string_slices_and_string_literals() {
    println!("=== string_slices_and_string_literals ===");
    let first_name = {
        let action_hero = "Arnold Schwarzenegger";
        &action_hero[0..6]
    };

    println!("{first_name}"); // Arnold

    let action_hero = "Arnold Schwarzenegger";
    let last_name = &action_hero[7..21];
    println!("{last_name}");
}

fn string_slice_lengths() {
    println!("=== string_slice_lengths ===");
    let food = "🍕";
    println!("{}", food.len());
    let pizza_slice = &food[0..4];
    println!("{}", pizza_slice.len());
}

fn syntactic_shortcuts() {
    println!("=== syntactic_shortcuts ===");
    let action_hero = String::from("Arnold Schwarzenegger");

    let first_name = &action_hero[..6];
    println!("His first name is {first_name}.");

    let last_name = &action_hero[7..];
    println!("His last name is {last_name}.");

    let full_name = &action_hero[..];
    println!("His full name is {full_name}.");
}

fn string_slices_as_function_parameters() {
    println!("=== string_slices_as_function_parameters ===");
    let action_hero = String::from("Arnold Schwarzenegger");
    _do_hero_stuff(&action_hero);
    let another_action_hero = "Sylvester Stallone";
    _do_hero_stuff(another_action_hero);
}

fn _do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day!");
}

fn array_slices() {
    println!("=== array_slices ===");
    let values = [4, 8, 15, 16, 23, 42];

    let my_slice = &values[..4];
    println!("{my_slice:?}");

    let my_slice = &values[2..4];
    println!("{my_slice:?}");

    let my_slice = &values[2..];
    println!("{my_slice:?}");

    let my_slice = &values[..];
    println!("{my_slice:?}");

    let my_slice = &values;
    println!("{my_slice:?}");
}

fn deref_coercion_with_array_slices() {
    println!("=== deref_coercion_with_array_slices ===");
    let values = [4, 8, 15, 16, 23, 42];

    let regular_reference = &values;
    _print_length(regular_reference);

    let slice_of_three = &values[0..3];
    _print_length(slice_of_three);
}

fn _print_length(slice: &[i32]) {
    println!("{}", slice.len());
}

fn mutable_array_slices() {
    println!("=== mutable_array_slices ===");
    let mut my_array = [10, 15, 20, 25, 30];
    println!("My array: {:?}", my_array);
    let my_slice: &mut [i32] = &mut my_array[2..4];
    println!("My slice: {:?}", my_slice);

    my_slice[0] = 100;
    println!("My slice: {:?}", my_slice);
    println!("My array: {:?}", my_array);
}

fn project() {
    /*
    Define a `cereals` array with 5 heap Strings
      - Cookie Crisp
      - Cinnamon Toast Crunch
      - Frosted Flakes
      - Cocoa Puffs
      - Captain Crunch

    Declare a `first_two` variable that extracts a slice
    of the first two cereals. Print the slice.

    Declare a `mid_three` variable that extracts a slice
    of the middle three cereals (Cinnamon Toast Crunch
    up to and including Cocoa Puffs). Print the slice.

    Declare a `last_three` variable that extracts a slice
    of the last three cereals. Print the slice.

    Using the `last_three` slice, target the last element
    ("Captain Crunch") and replace it with "Lucky Charms".
    Print the complete `cereals` array.

    Declare a `cookie_crisp` variable. Make it a reference
    to the "Cookie Crisp" String (in other words, a &String).

    Declare a `cookie` variable that extracts a slice of
    the text "Cookie" from the String. Print the slice.

    Declare a `cocoa_puffs` variable. Make it a reference
    to the "Cocoa Puffs" String (in other words, a &String).

    Declare a `puffs` variable that extracts a slice of
    the text "Puffs" from the String. Print the slice.
    */
    println!("=== project ===");

    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let first_two = &cereals[..2];
    println!("{first_two:?}");

    let mid_three = &cereals[1..4];
    println!("{mid_three:?}");

    let last_three = &mut cereals[2..];
    println!("{last_three:?}");
    last_three[2] = String::from("Lucky Charms");
    println!("{cereals:?}");

    let cookie_crisp = &cereals[0];
    let cookie = &cookie_crisp[..6];
    println!("{cookie}");

    let cocoa_puffs = &cereals[3];
    let puffs = &cocoa_puffs[6..];
    println!("{puffs}");
}
