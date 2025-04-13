use std::process;

#[allow(unused)]
pub fn main() {
    println!("=== no02_panic_macro ===");
    process::exit(1);
    println!("This will not print");
}
