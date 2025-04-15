use std::collections::HashMap;

fn count_characters(text: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::new();

    for character in text.chars() {
        let count = counts.entry(character).or_insert(0);
        *count += 1;
    }

    counts
}

fn count_words(text: &str) -> HashMap<&str, u32> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    counts
}

pub fn main() {
    println!("=== no10_solving_a_problem_with_iteration ===");
    println!(
        "{:?}",
        count_words("Sally sells sea shells by the sea shore.")
    );
    println!(
        "{:?}",
        count_characters("Sally sells sea shells by the sea shore.")
    );
}
