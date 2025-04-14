#![allow(unused)]

#[derive(Debug)]
struct TravelPlan<'a, 'b> {
    from: &'b str,
    to: &'a str,
}

pub fn main() {
    println!("=== no13_multiple_lifetimes ===");
    let from = String::from("Portland");
    let plan = figure_out_ending_point(&from);
    println!("{plan}");
}

fn figure_out_ending_point(from: &str) -> &str {
    let to = String::from("Bangor");

    let travel_plan = TravelPlan {
        from: &from,
        to: &to,
    };
    travel_plan.from
}
