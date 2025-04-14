#[derive(Debug)]
struct Location {
    name: String,
    treasures: u32,
}

#[derive(Debug)]
struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        let final_index = self.locations.len();
        let mut current_index = 0;
        while current_index < final_index {
            let current_location = &self.locations[current_index];
            action(current_location);
            current_index += 1;
        }
    }
}

pub fn main() {
    println!("=== no11_defining_a_method_that_accepts_a_closure_ii ===");
    let locations = [
        Location {
            name: String::from("Enchanted Forest"),
            treasures: 5,
        },
        Location {
            name: String::from("Mystic Mountain"),
            treasures: 10,
        },
    ];
    let map = Map {
        locations: &locations,
    };
    let mut total_treasures = 0;

    map.explore(|location| {
        total_treasures += location.treasures;
    });

    println!("Total treasures collected: {}", total_treasures);

    let mut location_names: Vec<String> = Vec::new();

    map.explore(|location| {
        location_names.push(location.name.clone());
    });
    println!("{location_names:?}");
}
