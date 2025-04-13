use std::fmt::{Display, Formatter, Result};

struct Apple {
    kind: String,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{} 🍏 for {}", self.kind, self.price)
    }
}

pub fn main() {
    println!("=== no06_associated_constants_in_a_trait ===");
    let lunch_snack = Apple {
        kind: String::from("Granny Smith"),
        price: 1.04,
    };

    println!("{}", lunch_snack);
}
