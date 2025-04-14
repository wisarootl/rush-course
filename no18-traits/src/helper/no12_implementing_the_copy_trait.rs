#![allow(unused)]

#[derive(Clone, Debug)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

impl Copy for Duration {}

pub fn main() {
    println!("=== no12_implementing_the_copy_trait ===");
    let one_hour = Duration::new(60, 0, 0);
    let another_hour = one_hour; // copy trait implement. no ownership transfer

    println!("{:?}", one_hour);
}
