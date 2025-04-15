#![allow(unused)]

#[derive(Debug)]
struct GasStation {
    snack_count: u32,
    manager: String,
    employee_count: u32,
}

pub fn main() {
    println!("=== no30_sort_and_sort_by_key_methods ===");

    let mut points = [3, 8, 1, 11, 5];
    println!("{}", points.is_sorted());

    points.sort();

    println!("{points:?}");
    println!("{}", points.is_sorted());

    points.reverse();
    println!("{points:?}");
    println!("{}", points.is_sorted());

    let mobil = GasStation {
        snack_count: 100,
        manager: String::from("Meg Mobil"),
        employee_count: 3,
    };

    let exxon = GasStation {
        snack_count: 130,
        manager: String::from("Eric Exxon"),
        employee_count: 4,
    };

    let shell = GasStation {
        snack_count: 50,
        manager: String::from("Shane Shell"),
        employee_count: 2,
    };

    let mut stops = [mobil, exxon, shell];

    stops.sort_by_key(|station| station.snack_count);
    println!("{stops:#?}");

    stops.sort_by_key(|station| -(station.employee_count as i32));
    println!("{stops:#?}");
}
