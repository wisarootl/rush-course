fn main() {
    define_a_struct();
    overwrite_struct_fields();
    create_structs_in_a_function();
    struct_update_syntax();
    passing_structs_into_a_function();
    deriving_debug_trait_for_struct();
    defining_struct_methods();
    self_parameter_as_mutable_struct_instance();
    self_parameter_as_immutable_and_mutable_references_to_struct_instance();
    methods_with_multiple_parameters();
    calling_methods_from_other_methods();
    associated_functions();
    builder_pattern();
    tuple_structs();
    unit_like_structs();
    project();
}

fn define_a_struct() {
    /*
    rust have 3 kinds of struct
    - named field structs
    - tuple-like structs
    - unit-like structs
    */
    println!("=== define_a_struct ===");

    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    println!(
        "My {} this morning cost {}. It is {} that it was hot.",
        mocha.name, mocha.price, mocha.is_hot
    );

    let favorite_coffee = mocha.name;
    println!("{favorite_coffee}");

    // println!("{}", mocha.name);
}

fn overwrite_struct_fields() {
    println!("=== overwrite_struct_fields ===");
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    let mut beverage: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    beverage.name = String::from("Caramel Macchiato");
    beverage.price = 6.99;
    beverage.is_hot = false;

    println!(
        "My {} this morning cost {}. It is {} that it was hot.",
        beverage.name, beverage.price, beverage.is_hot
    );
}

struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn create_structs_in_a_function() {
    println!("=== create_structs_in_a_function ===");
    let name = String::from("Latte");
    let coffee: Coffee = _make_coffee(name.clone(), 4.99, true);
    println!(
        "My {} this morning cost {}. It is {} that it was hot.",
        coffee.name, coffee.price, coffee.is_hot
    );

    let coffee: Coffee = _make_coffee_shorthand_syntax(name.clone(), 4.99, true);
    println!(
        "My {} this morning cost {}. It is {} that it was hot.",
        coffee.name, coffee.price, coffee.is_hot
    );

    let name = String::from("Latte");
    let price = 3.99;
    let is_hot = false;
    let latte = Coffee {
        name,
        price,
        is_hot,
    };
    println!(
        "My {} this morning cost {}. It is {} that it was hot.",
        latte.name, latte.price, latte.is_hot
    );
}

fn _make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name: name,
        price: price,
        is_hot: is_hot,
    }
}

fn _make_coffee_shorthand_syntax(name: String, price: f64, is_hot: bool) -> Coffee {
    // we can use shorthand syntax if parameter and variable names are the same
    Coffee {
        name,
        price,
        is_hot: is_hot,
    }
}

fn struct_update_syntax() {
    println!("=== struct_update_syntax ===");
    let mocha = _make_coffee(String::from("Mocha"), 4.99, true);

    let caramel_macchiato = Coffee {
        name: mocha.name.clone(),
        ..mocha
    };

    println!("{}", caramel_macchiato.name);
    println!("{}", mocha.name);
}

fn passing_structs_into_a_function() {
    println!("=== passing_structs_into_a_function ===");
    let mut mocha = _make_coffee(String::from("Mocha"), 4.99, true);
    _drink_coffee(&mut mocha);

    println!("{}", mocha.price);
}

fn _drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious {}", coffee.name); // behind the scene `coffee.name = (*coffee).name`
    println!("Drinking my delicious {}", (*coffee).name);
    coffee.is_hot = false;
    coffee.price = 10.99;
}

#[derive(Debug)]
struct CoffeeDebug {
    name: String,
    price: f64,
    is_hot: bool,
}

fn deriving_debug_trait_for_struct() {
    println!("=== deriving_debug_trait_for_struct ===");
    let mocha = CoffeeDebug {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };
    println!("debug display");
    println!("{:?}", mocha);
    println!("pretty-print display");
    println!("{:#?}", mocha);
    println!("{:#?}", mocha.name);
    println!("{:#?}", mocha.price);
    println!("{:#?}", mocha.is_hot);
}

#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    /*
    self param in struct method option
    - self
    - mut self
    - &self
    - &mut self
    */

    // immutable struct value (self parameter takes ownership)
    fn display_song_info(self) -> Self {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
        self
    }

    // Mutable struct value (self parameter takes ownership, has permission to mutate)
    fn double_length(mut self) -> Self {
        self.duration_secs = self.duration_secs * 2;
        println!("{:#?}", self);
        self
    }

    // Immutable reference to the struct instance (no ownership moved)
    fn display_song_info_immut_ref(&self) {
        println!("Title: {}", self.title);
        println!("Title: {}", (*self).title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    // Mutable reference to the struct instance (no ownership moved, have permission to mutate)
    fn double_length_mut_ref(&mut self) {
        self.duration_secs = self.duration_secs * 2;
        println!("Duration: {} seconds", self.duration_secs);
        self.duration_secs = (*self).duration_secs * 2;
        println!("Duration: {} seconds", self.duration_secs);
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn display_song_info_2(&self) {
        self.display_song_info_immut_ref();
        println!("Years Since Release: {}", self.years_since_release());
    }

    fn years_since_release(&self) -> u32 {
        2024 - self.release_year
    }
}

fn defining_struct_methods() {
    println!("=== defining_struct_methods ===");
    let song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    song.display_song_info();
}

fn self_parameter_as_mutable_struct_instance() {
    println!("=== self_parameter_as_mutable_struct_instance ===");
    let song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    // song.display_song_info(); // ownership move from song to self
    // song.double_length(); // self is invalid here

    let song = song.display_song_info();
    let song = song.double_length();
    println!("{:#?}", song);
}

fn self_parameter_as_immutable_and_mutable_references_to_struct_instance() {
    println!("=== self_parameter_as_immutable_and_mutable_references_to_struct_instance ===");
    let mut song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };
    song.display_song_info_immut_ref();
    song.double_length_mut_ref();
}

fn methods_with_multiple_parameters() {
    println!("=== methods_with_multiple_parameters ===");
    let blank_space = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    let all_too_well = TaylorSwiftSong {
        title: String::from("All Too Well"),
        release_year: 2012,
        duration_secs: 327,
    };

    if blank_space.is_longer_than(&all_too_well) {
        println!(
            "{} is longer than {}.",
            blank_space.title, all_too_well.title
        );
    } else {
        println!(
            "{} is shorter than or equal to {}.",
            blank_space.title, all_too_well.title
        );
    }
}

fn calling_methods_from_other_methods() {
    println!("=== calling_methods_from_other_methods ===");
    let blank_space = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    blank_space.display_song_info_2();
}

impl TaylorSwiftSong {
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        // this constructor function is example of associated functions
        TaylorSwiftSong {
            title,
            release_year,
            duration_secs,
        }
    }
}

fn associated_functions() {
    println!("=== associated_functions ===");
    let blank_space = TaylorSwiftSong::new(String::from("Blank Space"), 2014, 231);

    blank_space.display_song_info_immut_ref();

    TaylorSwiftSong::display_song_info(blank_space);
}
#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}

fn builder_pattern() {
    println!("=== builder_pattern ===");
    let mut computer = Computer::new(String::from("M3 Max"), 64, 2);

    computer
        .upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(128)
        .upgrade_hard_drive_capacity(3);

    println!("Stats: {computer:#?}");
}

// Hours, minutes
struct ShortDuration(i32, i32);
// Years, months
struct LongDuration(i32, i32);

fn tuple_structs() {
    println!("=== tuple_structs ===");
    let work_shift = ShortDuration(8, 30);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years {} months", era.0, era.1);

    // let work_shift = (8, 0);
    // let era = (5, 3);
    go_to_work(work_shift);
    // go_to_work(era); // invalid to pass era here

    let work_shift_tuple = (8, 30);
    let era_tuple = (5, 3);
    accept_tuple(work_shift_tuple);
    accept_tuple(era_tuple);
}

fn go_to_work(length: ShortDuration) {
    println!("Passing time {} hours {} minutes", length.0, length.1);
}

fn accept_tuple(length: (u32, u32)) {
    println!("tuple = ({},  {})", length.0, length.1);
}
struct Empty;

fn unit_like_structs() {
    let _empty = (); // unit
    let _my_empty_struct = Empty; // unit-like struct
}

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination;
    }

    fn increase_price(&mut self) {
        self.price *= 1.2;
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn project() {
    /*
    Define a Flight struct with the following fields:
    - an `origin` field (String)
    - a `destination` field (String)
    - a `price` field (f64)
    - a `passengers` field (u32)

    Derive a Debug trait implementation for the Flight struct.

    Define a `new` constructor function that returns a new
    instance of a Flight.

    Define a `change_destination` method that accepts a new
    destination and overwrites the value of the `destination`
    field.

    Define a `increase_price` method that raises the value
    of the `price` by 20% (multiply the `price` field by 1.20).
    Make sure to save the new `price` field value.

    Define a `itinerary` method that prints out both the
    `origin` and `destination` fields in the following format
    (origin -> destination).

    Use the constructor function to create a new Flight instance
    in the main function. Invoke all of the defined methods.
    Print out the struct in Debug format to confirm the struct
    updates as you expect.

    Use struct update syntax to copy the `price` and `passengers`
    fields to a new Flight struct instance. Make sure to provide
    new Strings for the remaining fields to ensure ownership
    doesn't transfer. Assign the new Flight to a separate variable.
    */
    println!("=== project ===");
    let mut my_flight = Flight::new(
        String::from("New York"),
        String::from("Los Angeles"),
        299.99,
        150,
    );

    println!("{:?}", my_flight);
    my_flight.change_destination(String::from("San Francisco"));
    my_flight.increase_price();
    my_flight.itinerary();
    println!("{:?}", my_flight);
    println!("{:?}", my_flight.passengers);

    let another_flight = Flight {
        origin: String::from("Paris"),
        destination: String::from("Rome"),
        ..my_flight
    };

    println!("{:#?}", another_flight);
    another_flight.itinerary();
}
