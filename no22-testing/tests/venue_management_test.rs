use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use no22_testing::helper::attractions::MovieTheater;
use no22_testing::helper::management::VenueManagement;
use no22_testing::helper::no01_intro_to_testing::Museum;

#[test]
fn venue_management_interacts_with_venue() {
    // this is integration test
    let mut museum = Museum::new();
    museum.buy_painting("Mona Lisa");

    let mut venue_mgmt = VenueManagement::new(museum);
    venue_mgmt.make_money();

    assert_eq!(venue_mgmt.venue.revenue, 25);
}

#[fixture]
fn museum_with_three_paintings() -> Museum {
    let mut museum = Museum::new();
    museum.buy_painting("Mona Lisa");
    museum.buy_painting("The Starry Night");
    museum.buy_painting("Girl with a Pearl Earring");
    museum
}

#[rstest]
fn venue_management_interacts_with_museum_venue(museum_with_three_paintings: Museum) {
    let mut venue_mgmt = VenueManagement::new(museum_with_three_paintings);
    venue_mgmt.make_money();

    assert_eq!(venue_mgmt.venue.paintings.len(), 3);
    assert_eq!(venue_mgmt.venue.revenue, 25);
}

#[fixture]
fn movie_theater_with_one_movie() -> MovieTheater {
    let mut movie_theater = MovieTheater::new();
    movie_theater.add_movie("Titanic");
    movie_theater
}

#[fixture]
fn movie_theater_management(
    movie_theater_with_one_movie: MovieTheater,
) -> VenueManagement<MovieTheater> {
    VenueManagement::new(movie_theater_with_one_movie)
}
