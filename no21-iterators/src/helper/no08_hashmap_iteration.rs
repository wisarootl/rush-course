use std::collections::HashMap;

pub fn main() {
    println!("=== no08_hashmap_iteration ===");

    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (todo, completion_status) in todos.clone() {
        println!("Task: {todo}. Complete: {completion_status}");
    }

    println!("{todos:?}");
    for (_, completion_status) in &mut todos {
        *completion_status = true;
    }

    println!("{todos:?}");
}
