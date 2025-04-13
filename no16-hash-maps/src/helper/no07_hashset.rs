use std::collections::HashSet;

pub fn main() {
    println!("=== no07_hashset ===");

    let mut concert_queue: HashSet<&str> = HashSet::new();
    println!("{:?}", concert_queue);

    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    println!("{:?}", concert_queue);
    println!("{}", concert_queue.len());

    concert_queue.insert("Molly");
    println!("{:?}", concert_queue);

    println!("{}", concert_queue.remove("Megan"));
    println!("{}", concert_queue.remove("Franny"));
    println!("{:?}", concert_queue);

    println!("{}", concert_queue.contains("Molly"));
    println!("{}", concert_queue.contains("Fred"));

    println!("{:?}", concert_queue.get("Molly"));
    println!("{:?}", concert_queue.get("Joe"));
}
