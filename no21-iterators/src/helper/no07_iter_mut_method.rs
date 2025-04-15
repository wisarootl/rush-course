pub fn main() {
    println!("=== no07_iter_mut_method ===");

    let mut flavors = [
        String::from("Chocolate"),
        String::from("Vanilla"),
        String::from("Strawberry"),
    ];

    let iterator = flavors.iter_mut();

    for flavor in iterator {
        flavor.push_str(" Ice Cream");
    }

    println!("{flavors:?}");
    println!("\n===");

    for flavor in &mut flavors {
        // implicit call .iter_mut()
        flavor.push_str(" 2");
    }

    println!("{flavors:?}");
    println!("\n===");

    let mut school_grades = [85, 90, 78, 92];
    println!("{school_grades:?}");
    for grade in &mut school_grades {
        *grade -= 2;
    }

    println!("{school_grades:?}");
    println!("===");
}

// OWNERSHIP
// for value in collection
// for value in collection.into_iter()

// IMMUTABLE REFERENCES
// for value in &collection
// for value in collection.iter()

// MUTABLE REFERENCES
// for value in &mut collection
// for value in collection.iter_mut()
