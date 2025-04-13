use warehouse::inventory::FLOOR_SPACE;
use warehouse::{INVENTORY_MANAGER, ORDERS_MANAGER};

// Get the summary of our current manager
fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space.",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );
}
