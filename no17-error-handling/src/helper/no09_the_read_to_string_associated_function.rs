use std::fs;
use std::io::{self, stdin};

#[allow(unused)]
pub fn main() {
    println!("=== no09_the_read_to_string_associated_function ===");
    let file_result = read_file();

    match file_result {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an error: {error:?}");
        }
    }
}

fn read_file() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read:");

    let mut input = String::new();
    stdin().read_line(&mut input)?; // capture Result::Ok return Result::Err

    fs::read_to_string(input.trim())
}
