#![allow(unused)]

use std::io::stdin;

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    fn unlock(self, procedure: impl FnOnce() -> String) -> Option<String> {
        let password = procedure();
        if password == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }
}

pub fn main() {
    println!("=== no09_defining_a_method_that_accepts_a_closure_i ===");

    let vault = Vault {
        password: String::from("topsecret"),
        treasure: String::from("Gold"),
    };

    let hack = || {
        let mut user_input = String::new();
        println!("Please provide password to crack vault");
        stdin().read_line(&mut user_input);
        user_input.trim().to_string()
    };
    let extraction = vault.unlock(hack);

    println!("{:?}", extraction);
}
