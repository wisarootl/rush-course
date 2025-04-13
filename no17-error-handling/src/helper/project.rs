use std::fs;
use std::io;
use std::process;

pub fn main() {
    /*
    Define a `write_to_file` function. The function
    should ask the user the following questions:

    What file would you like to write to?
    What would you like to write to the file?

    Collect the user's two entries as Strings. If something
    fails in either collection, propagate the error upwards;
    the main function (the caller) will handle the error
    later.

    Then, use the file system module's `write` function
    to write the user's specified contents to their
    requested text file. The documentation for `write`
    can be  found here:
    https://doc.rust-lang.org/std/fs/fn.write.html

    If the `write` function fails, propagate the error
    upwards as well.

    Your `write_to_file` function should return an
    `io::Result`.

    After you write to the file, return a `Ok` variant
    storing the user's requested file name.

    In the main function, use a match statement to react
    to both variants of the returned Result enum. If
    everything worked, output the text "Successfully
    wrote to file { }" and interpolate the file name you
    collected in the `write_to_file` function.

    If there was any failure, output "There was an error:
    {error}" to the standard error output and
    interpolate the error. Then, exit the program with a
    status code of 1.
     */
    println!("=== project ===");
    match write_to_file() {
        Ok(file_name) => println!("Successfully wrote to file {file_name}"),
        Err(error) => {
            eprintln!("There was an error: {error}");
            process::exit(1);
        }
    }
}

fn write_to_file() -> io::Result<String> {
    let input = io::stdin();

    println!("What file would you like to write to?");
    let mut requested_file = String::new();
    input.read_line(&mut requested_file)?;

    println!("What would you like to write to the file?");
    let mut content = String::new();
    input.read_line(&mut content)?;

    fs::write(requested_file.trim(), content.trim())?;

    Ok(requested_file)
}
