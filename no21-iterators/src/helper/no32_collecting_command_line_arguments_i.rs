use std::env;

pub fn main() {
    println!("=== no32_collecting_command_line_arguments_i ===");
    let args = env::args();

    for arg in args {
        println!("{arg}");
    }
    // please try running with
    /*
    ```
    cargo run -- hello world
    ```

    output
    ```
    === no32_collecting_command_line_arguments_i ===
    target/debug/no21-iterators
    hello
    world
    ```
    */
}
