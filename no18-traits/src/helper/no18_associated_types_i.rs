use std::ops::Add;

#[derive(Debug)]
struct Lunch {
    cost: f64,
}

impl Add for Lunch {
    type Output = Lunch;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }
}

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn main() {
    println!("=== no18_associated_types_i ===");
    let one = Lunch { cost: 19.99 };
    let two = Lunch { cost: 29.99 };
    println!("{:?}", one + two);

    let integer_sum = add_two_numbers(1, 2);
    let float_sum = add_two_numbers(1.5, 2.4);
    println!("{integer_sum} and {float_sum}");
}
