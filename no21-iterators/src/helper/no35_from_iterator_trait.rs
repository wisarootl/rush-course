#![allow(unused)]

use std::collections::HashSet;

#[derive(Debug)]
struct Playlist {
    songs: Vec<String>,
    users: HashSet<String>,
}

impl FromIterator<(String, String)> for Playlist {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }
        Self { songs, users }
    }
}
pub fn main() {
    println!("=== no35_from_iterator_trait ===");

    let fifty_numbers = 1..=10;

    let results = Vec::from_iter(fifty_numbers.clone());
    println!("{results:?}");

    let results: Vec<_> = fifty_numbers.clone().collect();
    println!("{results:?}");

    let unique_set: HashSet<_> = HashSet::from_iter(fifty_numbers.clone());
    println!("{unique_set:?}");

    let unique_set = fifty_numbers.clone().collect::<HashSet<i32>>();
    println!("{unique_set:?}");

    let songs = [
        (String::from("I Rust Go On"), String::from("Bob")),
        (String::from("A Rust of Wind"), String::from("Bob")),
        (String::from("A Rustworthy Man"), String::from("Sheila")),
    ];

    let playlist: Playlist = Playlist::from_iter(songs.clone());
    println!("{playlist:?}");

    let playlist = songs.clone().into_iter().collect::<Playlist>();
    println!("{playlist:?}");
}
