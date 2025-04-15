pub fn main() {
    println!("=== no21_flat_map_method ===");
    let attendees = [
        "Bob, Mary, Kevin",
        "Mike, Robbie, Matt, Austin",
        "Piers, Liam",
    ];

    let attendees: Vec<_> = attendees
        .iter()
        .map(|group| group.split(", "))
        .flatten()
        .collect();
    println!("{attendees:?}");

    let attendees: Vec<_> = attendees
        .iter()
        .flat_map(|group| group.split(", "))
        .collect();

    println!("{attendees:?}");
}
