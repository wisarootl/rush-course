#![allow(unused)]

use std::collections::HashMap;

pub fn main() {
    println!("=== no02_into_iterator_trait_in_action ===");
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.into_iter(); // my_vector ownership is moved here to my_iterator
    // println!("{my_vector}"); // invalid

    let my_vector = vec![false, true, false];
    let my_iterator = my_vector.into_iter();

    let mut my_hashmap = HashMap::new();
    my_hashmap.insert("CBS", 2);
    let my_iterator = my_hashmap.into_iter();
}
