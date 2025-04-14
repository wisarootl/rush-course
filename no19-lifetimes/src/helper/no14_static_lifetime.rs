const COUNT: i32 = 400;

//  'static is for ref of constant or literal string (embedfed in execute binary)
// the one that will be existed to the whole program.

fn say_hello() -> &'static str {
    "Hello"
}

fn value() -> &'static i32 {
    &COUNT
}

pub fn main() {
    println!("=== no14_static_lifetime ===");
    let greeting = say_hello();
    println!("{greeting}");

    let value = value();
    println!("{value}");
}
