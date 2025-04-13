/// Tools for inventory management
pub mod inventory;
/// Tools for order manager
pub mod orders;

pub use inventory::MANAGER as INVENTORY_MANAGER;
pub use orders::MANAGER as ORDERS_MANAGER;
