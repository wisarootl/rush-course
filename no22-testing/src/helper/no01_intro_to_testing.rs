#![allow(unused)]

// mod crate::helper::attractions;

use crate::helper::attractions::TicketSeller;

// lib.rs
#[derive(Debug, Eq, PartialEq)]
pub struct Museum {
    pub paintings: Vec<String>,
    pub revenue: u32,
}

impl Museum {
    const MAXIMUM_CAPACITY: usize = 3;

    /// Create a new Museum instance.
    ///
    /// # Examples:
    /// ```
    /// use no22_testing::helper::no01_intro_to_testing::Museum;
    ///
    /// let museum = Museum::new();
    /// let empty_vec: Vec<String> = Vec::new();
    /// assert_eq!(museum.paintings, empty_vec);
    /// assert_eq!(museum.revenue, 0);
    /// ```
    pub fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    /// Buys a painting for the museum.
    ///
    /// # Examples
    /// ```
    /// use no22_testing::helper::no01_intro_to_testing::Museum;
    ///
    /// let mut museum = Museum::new();
    /// museum.buy_painting("Mona Lisa");
    /// assert_eq!(museum.paintings, vec!["Mona Lisa".to_string()])
    /// ```
    pub fn buy_painting(&mut self, painting: &str) {
        if self.paintings.len() >= Self::MAXIMUM_CAPACITY {
            panic!("Museum does not have storage space for another painting!");
        }

        self.paintings.push(painting.to_string());
    }

    fn has_impressive_collection(&self) -> bool {
        self.paintings.len() > 2
    }
}

impl TicketSeller for Museum {
    fn sell_ticket(&mut self) {
        self.revenue += 25;
    }
}

pub fn main() {
    println!("=== no01_intro_to_testing ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn museum_sells_ticket_to_increase_revenue() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 25);
    }

    #[test]
    fn museum_can_sell_multiple_tickets() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 50);
    }

    #[test]
    fn museum_can_have_impressive_art_collection() {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        assert!(!museum.has_impressive_collection());
        museum.buy_painting("The Starry Night");
        assert!(!museum.has_impressive_collection());
        museum.buy_painting("Girl with a Pearl Earring");
        assert!(museum.has_impressive_collection());
    }

    #[test]
    fn museum_sells_ticket_to_increase_revenue_2() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_ne!(museum.revenue, 0);
    }

    #[test]
    fn hashmaps() {
        let mut one = std::collections::HashMap::new();
        one.insert("A", 2);
        one.insert("B", 3);

        let mut two = std::collections::HashMap::new();
        two.insert("B", 3);
        two.insert("C", 4);

        // assert_eq!(one, two);
    }

    #[test]
    fn new_museums_are_equal() {
        let museum1 = Museum::new();
        let museum2 = Museum::new();
        assert_eq!(museum1, museum2);

        let mut museum1 = Museum::new();
        museum1.sell_ticket();
        let museum2 = Museum::new();
        assert_ne!(
            museum1, museum2,
            "Two new Museum instances were not found to be equal: {museum1:?} // {museum2:?}"
        );
    }

    #[test]
    #[should_panic(expected = "storage space")]
    fn it_prohibits_adding_painting_when_capacity_has_been_reached() {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");
        museum.buy_painting("Water Lilies");
    }

    #[test]
    fn result_example() -> Result<(), String> {
        if true {
            Ok(())
        } else {
            Err(String::from("Failure"))
        }
    }

    #[test]
    fn print_success() {
        // `cargo test -- --show-output` to show success output
        println!("Success inside the function");
        assert!(true);
    }

    #[test]
    fn print_failure() {
        println!("Failure inside the function");
        // assert!(false);
    }
}
