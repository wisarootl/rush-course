#![allow(unused)]

use crate::helper::attractions::{MovieTheater, TicketSeller};

#[derive(Debug)]
pub struct VenueManagement<T: TicketSeller> {
    pub venue: T,
    manager: Option<String>,
}

impl<T: TicketSeller> VenueManagement<T> {
    pub fn new(venue: T) -> Self {
        Self {
            venue,
            manager: None,
        }
    }

    pub fn hire_manager(&mut self, manager: &str) {
        self.manager = Some(manager.to_string());
    }

    pub fn make_money(&mut self) {
        self.venue.sell_ticket();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    struct DummyVenue {}

    impl TicketSeller for DummyVenue {
        fn sell_ticket(&mut self) {}
    }
    #[test]
    fn venue_management_can_hire_manager() {
        let dummy_venue = DummyVenue {};
        let mut venue_mgmt = VenueManagement::new(dummy_venue);
        venue_mgmt.hire_manager("Mario");
        assert_eq!(venue_mgmt.manager.unwrap(), String::from("Mario"));
    }
}
