fn main() {
    create_a_vector();
    adding_and_removing_elements();
    reading_vector_elements();
    get_method();
    ownership_with_vectors();
    writing_vector_elements();
    vector_capacity_behind_the_scenes();
    project();
}

fn create_a_vector() {
    println!("=== create_a_vector ===");
    // array
    let risk_moranis_movies: [&str; 3] = ["Ghostbusters", "Honey, I Shrunk the Kids", "Spaceballs"];
    println!("{risk_moranis_movies:?}");

    // vector
    let pizza_diameters: Vec<i32> = Vec::new();
    println!("{pizza_diameters:?}");
    let pizza_diameters = Vec::<i32>::new();
    println!("{pizza_diameters:?}");

    let pizza_diameters: Vec<i32> = vec![8, 10, 12, 14];
    println!("{pizza_diameters:?}");

    let pastas: Vec<&str> = vec!["Rigatoni", "Angel hair", "Fettucine"];
    println!("{pastas:?}");
}

fn adding_and_removing_elements() {
    println!("=== adding_and_removing_elements ===");
    let mut pizza_diameters = vec![8, 10, 12, 14];
    pizza_diameters.push(16);
    println!("{pizza_diameters:?}");
    pizza_diameters.push(18);
    println!("{pizza_diameters:?}");

    pizza_diameters.insert(0, 4);
    println!("{pizza_diameters:?}");

    let last_pizza_diameter = pizza_diameters.pop();
    println!("{last_pizza_diameter:?}");
    println!("{pizza_diameters:?}");

    let third_diameter_from_start = pizza_diameters.remove(2);
    println!("{third_diameter_from_start}"); // 10
    println!("{pizza_diameters:?}");

    // pizza_diameters.remove(50);
}

fn reading_vector_elements() {
    println!("=== reading_vector_elements ===");
    let pizza_diameters = vec![8, 10, 12, 14];
    let value = pizza_diameters[2];
    println!("{value:?}");

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];
    // println!("{pepperoni:?}"); // ownership is moved to the vector

    let pizza_slice = &pizza_toppings[1..];
    println!("{pizza_slice:?}");
}

fn _handle_option(option: Option<&String>) {
    match option {
        Some(topping) => println!("The topping is {topping}"),
        None => println!("No value at that index position"),
    }
}

fn get_method() {
    println!("=== get_method ===");
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    let option = pizza_toppings.get(2);
    _handle_option(option);

    let option = pizza_toppings.get(50);
    _handle_option(option);
}

fn ownership_with_vectors() {
    println!("=== ownership_with_vectors ===");
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];
    let mut delicious_toppings = pizza_toppings;
    // println!("{pizza_toppings:?}"); // owner is moved

    let topping_reference = &delicious_toppings[1];
    println!("The topping is {topping_reference}");

    delicious_toppings.push(String::from("Olives"));
}

fn writing_vector_elements() {
    println!("=== writing_vector_elements ===");
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let mut pizza_toppings = vec![pepperoni, mushroom, sausage];

    pizza_toppings[1] = String::from("Olives");
    println!("{pizza_toppings:#?}");

    let target_topping = &mut pizza_toppings[2];
    target_topping.push_str(" and Meatballs");
    let another_topping = &pizza_toppings[2];
    let another_one = &pizza_toppings[2];
    println!("{another_topping}, {another_one}");
    println!("{pizza_toppings:#?}");
}

fn vector_capacity_behind_the_scenes() {
    println!("=== vector_capacity_behind_the_scenes ===");
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!(
        "Length: {}. Capacity: {}",
        seasons.len(),
        seasons.capacity(),
    );

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");
    println!(
        "Length: {}. Capacity: {}",
        seasons.len(),
        seasons.capacity(),
    );

    seasons.push("Summer");

    println!(
        "Length: {}. Capacity: {}",
        seasons.len(),
        seasons.capacity(),
    );
}

#[allow(dead_code)]
#[derive(Debug)]
struct File {
    name: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

fn handle_get_file(result: Option<&File>) {
    match result {
        Some(file) => println!("Retrieved file: {file:?}"),
        None => println!("There was no file"),
    }
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            contents: vec![],
        }
    }

    fn create_file(&mut self, name: String) {
        let file = File { name };
        self.contents.push(file);
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn project() {
    /*
    Let's model a file system on a computer.

    Define a File struct with a `name` field set to a
    String. Derive a Debug implementation.

    Define a Folder struct with a `name` field set to
    a String and a `contents` field set to a vector of
    File structs. Derive a Debug implementation.

    On the Folder struct...

    Define a `new` constructor function that accepts a
    `name` String. The method should create and return
    a new Folder with that name. For the `contents` field,
    provide a hardcoded empty vector.

    Define a `create_file` method that accepts a `name`
    String. The method should create a new File with that
    name and add it to the end of the `contents` vector.

    Define a `delete_file` method that accepts an `index`
    parameter of type `usize`. The method should remove the
    File at the specified index position from the `contents`
    vector. It should also return the File.

    Define a `get_file` method that accepts an `index`
    parameter of type `usize`. The method should return
    an Option containing a reference to the File at
    that index position.

    In the `main` function, use the `new` function to
    create a Folder instance with a `name` of your choosing.

    Call the `create_file` method two times. Print out
    the Folder in Debug format.

    Delete one of the two files using the `delete_file`
    method. Print out the Folder in Debug format.

    Call the `get_file` method. Use a match statement
    to react to both Option variants. For the Some variant,
    print out the File in Debug format. For the None variant,
    print out the text "There was no file".
     */
    println!("=== project ===");
    let mut folder = Folder::new(String::from("Documents"));

    folder.create_file(String::from("main.rs"));
    folder.create_file(String::from("lib.rs"));
    println!("{:#?}", folder);

    handle_get_file(folder.get_file(1));

    folder.delete_file(1);
    println!("{:#?}", folder);
    handle_get_file(folder.get_file(5));
}
