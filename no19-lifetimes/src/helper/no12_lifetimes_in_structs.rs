#[derive(Debug)]
struct TrainSystem<'a> {
    name: &'a str,
}

pub fn main() {
    println!("=== no12_lifetimes_in_structs ===");
    let name = String::from("AmTrak");
    let nj_transit = { TrainSystem { name: &name } };

    println!("{:#?}", nj_transit.name);
}
