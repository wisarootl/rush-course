use std::fs::File;
use std::io::stdin;
use std::process;

#[allow(unused)]
pub fn main() {
    println!("=== no05_asking_the_user_for_input ===");
    println!("Please enter the name of the file you'd like to read (example: `story.txt`):");
    let mut input = String::new();

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        eprintln!("1 Something went wrong collecting user input. The error was {error:?}");
        process::exit(1);
    }

    let file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("2 Something went wrong reading the file. The error was {error:?}");
            process::exit(1);
        }
    };

    println!("{file:?}");
}
