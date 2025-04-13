#![allow(unused)]

/*
Submodule options
- inline
- inventory/products.rs
- inventory/products/mod.rs
*/
pub mod products;

pub use products::{Item, ProductCategory};

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "John Doe";

pub fn talk_to_manager() {
    println!("Hey, {MANAGER}, how's your coffee?"); // relative paths
    println!("Hey, {}, how's your coffee?", crate::inventory::MANAGER); // absolute paths
    println!("{:?}", products::ProductCategory::Ladder); // relative paths
}

fn talk_to_manager_private() {
    println!("PRIVATE: Hey, {MANAGER}, how's your coffee?"); // relative paths
}
