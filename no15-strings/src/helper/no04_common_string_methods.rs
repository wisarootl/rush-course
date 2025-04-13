pub fn main() {
    println!("=== no04_common_string_methods ===");

    let mut music_genres = "   Rock, Metal, Country, Rap   ";
    println!("{}", music_genres.trim());
    println!("{}", music_genres.trim_start());
    println!("{}", music_genres.trim_end());

    music_genres = music_genres.trim();
    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());

    println!("{}", music_genres.replace("a", "@"));

    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{:?}", genres);
}
