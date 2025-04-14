use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
enum AppleType {
    RedDelicious,
    GrannySmith,
}

impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "🍎 Delicious 🍎"),
            AppleType::GrannySmith => write!(formatter, "🍏 Granny Smith 🍏"),
        }
    }
}

#[derive(Debug)]
struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{} for {}", self.kind, self.price)
    }
}

pub fn main() {
    println!("=== no07_implementing_the_display_trait_on_an_enum ===");
    let lunch_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };

    let dinner_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 1.15,
    };

    println!("{}", lunch_snack);
    println!("{}", dinner_snack);
    println!("{:?}", lunch_snack);
    println!("{:?}", dinner_snack);
}
