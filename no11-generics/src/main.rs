#[derive(Debug)]
struct DeliSandwich {}

fn main() {
    intro_to_generics();
    turbofish_operator();
    multiple_generics();
    generics_in_structs();
    generics_and_impl_blocks();
    generics_in_enums();
    project();
}

fn intro_to_generics() {
    println!("=== intro_to_generics ===");
    println!("{}", _identity(5));
    println!("{}", _identity(13.14));
    println!("{}", _identity("hello"));
    println!("{}", _identity(String::from("hello")));
    println!("{}", _identity(true));
    println!("{:?}", _identity(DeliSandwich {}));
}

fn _identity<T>(value: T) -> T {
    value
}

fn turbofish_operator() {
    println!("=== turbofish_operator ===");
    println!("{}", _identity::<i32>(5)); // explicit define concrete type for generic
    println!("{}", _identity::<i8>(5));
    println!("{}", _identity::<u32>(5));
    println!("{}", _identity::<f64>(13.14));
    println!("{}", _identity::<&str>("hello"));
    println!("{}", _identity::<String>(String::from("hello")));
    println!("{}", _identity::<bool>(true));
    println!("{:?}", _identity::<DeliSandwich>(DeliSandwich {}));
}

fn _make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

// same types
// fn _make_tuple<T>(first: T, second: T) -> (T, T) {
//     (first, second)
// }

fn multiple_generics() {
    println!("=== multiple_generics ===");
    _make_tuple(5, "hello");
    _make_tuple(5, 13);
    _make_tuple(true, 3.14);
    _make_tuple(true, false);
}

#[allow(dead_code)]
#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

fn generics_in_structs() {
    println!("=== generics_in_structs ===");

    let gold_chest = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold", // T is &str
    };
    println!("{:?}", gold_chest);

    let silver_chest = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: String::from("Silver"), // T is String
    };
    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("Bootyplunder"),
        treasure: ["Gold", "Silver", "Platinum"], // T is array [&str; 3]
    };
    println!("{:?}", special_chest);
}

// specify concrete type when implement struct with generic
impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

fn generics_and_impl_blocks() {
    println!("=== generics_and_impl_blocks_1 ===");
    let gold_chest = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    println!("{}", gold_chest.capital_captain());

    let mut silver_chest = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: String::from("     Silver    "),
    };
    silver_chest.clean_treasure();
    println!("{}", silver_chest.capital_captain());

    let special_chest = TreasureChest {
        captain: String::from("Bootyplunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{:?}", special_chest.amount_of_treasure());
    println!("{:?}", special_chest.capital_captain());
    println!("{:?}", special_chest);
}

enum Cheesesteak<T> {
    Plain,
    Topping(T),
}

#[allow(unused_assignments)]
#[allow(unused_variables)]
fn generics_in_enums() {
    // #![allow(unused_variables)]
    println!("=== generics_in_enums ===");
    let mushroom = Cheesesteak::Topping("mushroom");
    let onions = Cheesesteak::Topping("onions".to_string());
    let topping = "bacon".to_string();
    let bacon = Cheesesteak::Topping(&topping);

    let mut plain: Cheesesteak<String> = Cheesesteak::Plain;

    plain = Cheesesteak::Topping(String::from("sausage"));
    plain = Cheesesteak::Topping("sausage".to_string());

    // Invalid, &str is not a String, which is what T must be for plain variable
    // plain = Cheesesteak::Topping("sausage");
}

#[allow(dead_code)]
fn project() {
    /*
    Let's model a real-time chat system where users can
    share audio and video files.

    Define a DigitalContent enum with two variants:
    AudioFile and VideoFile. Derive a Debug implementation.

    Define a ChatMessage struct with two fields: `content`
    and `time`. The struct should define one generic type, T,
    which will be the type of the `content` field.
    The `time` field should always be a String.
    Derive a Debug implementation.

    Add an impl block for ChatMessage structs whose T type
    is a DigitalContent enum. Define a `consume_entertainment`
    method that prints out the value of the `content` field in
    Debug format. For example, "Watching the AudioFile".

    Add an impl block for ChatMessage structs with any type T.
    Define a `retrieve_time` method that returns a String.
    It should return a clone of the `time` field from
    the struct.

    In `main`, create a ChatMessage with `content` set to a
    string slice.

    Create another ChatMessage with `content` set to a String.

    Create another ChatMessage with `content' set to a
    DigitalContent variant.

    Invoke the `consume_entertainment` method on the
    ChatMessage storing a DigitalContent enum.

    Invoke the `retrieve_time` method on all 3 ChatMessage
    instances and print out each String's content.
    */

    #[derive(Debug)]
    enum DigitalContent {
        AudioFile,
        VideoFile,
    }

    #[derive(Debug)]
    struct ChatMessage<T> {
        content: T,
        time: String,
    }

    impl ChatMessage<DigitalContent> {
        fn consume_entertainment(&self) {
            println!("Watching the {:?}", self.content)
        }
    }

    impl<T> ChatMessage<T> {
        fn retrieve_time(&self) -> String {
            self.time.clone()
        }
    }

    let message = ChatMessage {
        content: "Hi, lol",
        time: String::from("2025-03-12"),
    };
    println!("{}", message.retrieve_time());

    let notification = ChatMessage {
        content: String::from("What's your favorite pizza topping?"),
        time: String::from("2025-04-12"),
    };
    println!("{}", notification.retrieve_time());

    let audio = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("2025-05-12"),
    };
    audio.consume_entertainment();
    println!("{}", audio.retrieve_time());
}
