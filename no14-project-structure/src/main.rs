/*
Submodule options
- inline
- inventory.rs
- inventory/mod.rs
*/

mod inventory;
mod orders;

// use warehouse::{inventory, orders};
use warehouse;

#[allow(unused)]
use std::{
    fmt,
    io::{self, stdin, stdout},
};

// use std::collections::*; // glob
// use std::prelude::v1::*; // default rust prelude

use fake::{Fake, Faker};

// use inventory::MANAGER;
// use inventory::products::{Item, ProductCategory};
use inventory::{FLOOR_SPACE, MANAGER as INVENTORY_MANAGER, talk_to_manager};
use orders::MANAGER as ORDERS_MANAGER;

use inventory::products::{self, Item, ProductCategory}; // self refer to products

mod inline_module {
    pub const VAR: &str = "FOO";
}

/// Primary entrypoint into our warehouse program
fn main() {
    println!("{}", inline_module::VAR);
    println!("The manager of our inventory is {}", inventory::MANAGER);
    println!("The manager of our orders is {}", orders::MANAGER);
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space.",
        inventory::MANAGER,
        orders::MANAGER,
        FLOOR_SPACE
    );

    talk_to_manager();

    let favorite_category = ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}.");

    let favorite_category = products::ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}.");

    let tall_ladder = Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 100,
    };

    println!("{:#?}", tall_ladder);

    let tall_ladder = Item::new(
        String::from("Ladder-o-matic 2000"),
        ProductCategory::Ladder,
        100,
    );

    println!("{:#?}", tall_ladder);

    create_aliases_with_the_as_keyword();
    using_pub_use_to_export_names_from_submodules();
    external_crates();
    create_library_crate();
}

fn create_aliases_with_the_as_keyword() {
    println!("The manager of our inventory is {}", INVENTORY_MANAGER);
    println!("The manager of our orders is {}", ORDERS_MANAGER);
}

fn using_pub_use_to_export_names_from_submodules() {
    println!("=== using_pub_use_to_export_names_from_submodules ===");
    let tall_ladder = inventory::Item::new(
        String::from("Ladder-o-matic 2000"),
        inventory::ProductCategory::Ladder,
        100,
    );

    println!("{:#?}", tall_ladder);
}

fn external_crates() {
    println!("=== external_crates ===");

    let fake_item: Item = Faker.fake();
    println!("{:?}", fake_item);

    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category);
}

fn create_library_crate() {
    println!("=== create_library_crate ===");

    println!(
        "The manager of our inventory is {}",
        warehouse::inventory::MANAGER
    );
    println!(
        "The manager of our orders is {}",
        warehouse::orders::MANAGER
    );
}
