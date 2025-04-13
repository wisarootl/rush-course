use std::collections::HashSet;

pub fn main() {
    println!("=== no08_hashset_operations ===");
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");

    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    println!("{:?}", concert_queue.union(&movie_queue));
    println!("{:?}", movie_queue.union(&concert_queue));

    println!("{:?}", concert_queue.difference(&movie_queue)); // A - B
    println!("{:?}", movie_queue.difference(&concert_queue));

    println!("{:?}", concert_queue.symmetric_difference(&movie_queue)); // (A ∪ B) - (A ∩ B)
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));

    println!("{}", concert_queue.is_disjoint(&movie_queue)); // A ∩ B == {}
    println!("{}", movie_queue.is_disjoint(&concert_queue));

    let mut attendees = HashSet::new();
    attendees.insert("Boris");
    println!("{}", attendees.is_subset(&concert_queue)); // A ⊆ B

    println!("{}", concert_queue.is_superset(&attendees));
}
