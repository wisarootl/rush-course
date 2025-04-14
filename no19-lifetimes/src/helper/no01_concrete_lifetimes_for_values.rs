#![allow(unused)]

pub fn main() {
    println!("=== no01_concrete_lifetimes_for_values ===");
    let a = 1; // `a` lifetime starts here

    {
        let b = 2; // `b` lifetime starts here
        // ...
        // `b` lifetime ends here
    }

    let c = String::from("Winter"); // `c` lifetime starts here
    let d = c; // `c` lifetime ends here
    // drop(c);
    // `a` lifetime ends here
}
