use std::fs;
use std::io;

pub fn main() {
    println!("=== no31_lines_method ===");
    _ = read_from_file();
    _ = read_from_raw_str();
    _ = read_from_escaped_str();
}

fn process_lines(contents: &str) -> Result<(), io::Error> {
    for line in contents.lines() {
        println!("{line}");
    }
    Ok(())
}

pub fn read_from_file() -> Result<(), io::Error> {
    let contents = fs::read_to_string("story.txt")?;
    process_lines(&contents)
}

pub fn read_from_raw_str() -> Result<(), io::Error> {
    let contents = r#"This is the first line.
This is the second line.
This is the third line."#;
    process_lines(&contents)
}

pub fn read_from_escaped_str() -> Result<(), io::Error> {
    let contents = "This is the first line.\n\
    This is the second line.\n\
    This is the third line.";
    process_lines(&contents)
}
