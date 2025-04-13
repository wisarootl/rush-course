use std::fs::File; // fs=filesystem
use std::process;

fn handle_file(file_result: Result<File, std::io::Error>) -> File {
    match file_result {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong reading the file. The error was {error:?}");
            process::exit(1);
        }
    }
}

#[allow(unused)]
pub fn main() {
    println!("=== no04_opening_a_file ===");

    let file = handle_file(File::open("story.txt"));
    println!("{file:#?}");
    let file = handle_file(File::open("not_exist.txt"));
    println!("{file:#?}");
}
