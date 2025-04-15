use std::fs;
use std::io;

pub fn main() {
    println!("=== no34_reading_directory ===");
    _ = main2();
}

pub fn main2() -> io::Result<()> {
    for entry_result in fs::read_dir("./")? {
        let entry = entry_result?;
        let entry_name = entry.path();
        let metadata = fs::metadata(&entry_name)?;
        if metadata.is_file() {
            println!("{entry_name:?}\n-------------");
            let contents = fs::read_to_string(&entry_name)?;
            // println!("{contents}");

            let first_five_lines: Vec<&str> = contents.lines().take(5).collect();
            for line in first_five_lines {
                println!("{line}");
            }
        }
    }

    Ok(())
}
