use std::fs::File;
use std::io::{Read, stdin};
use std::process;

#[allow(unused)]
pub fn main() {
    println!("=== no06_reading_the_files_contents ===");

    println!("Please enter the name of the file you'd like to read (example: `story.txt`):");
    let mut input = String::new();

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        eprintln!("Something went wrong collecting user input. The error was {error:?}");
        process::exit(1);
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong opening the file. The error was {error:?}");
            process::exit(1);
        }
    };

    let mut file_contents = String::new();
    let read_operation = file.read_to_string(&mut file_contents);

    if let Err(error) = read_operation {
        eprintln!("Something went wrong reading the file as a string. The error was {error}");
        process::exit(1);
    }

    println!("{file_contents}");
}
