pub fn main() {
    println!("=== no08_unwrap_or_else_method ===");

    let option = Some("Salami");
    // let option = None;
    let closure = || "Pizza";
    let food = option.unwrap_or_else(closure);
    println!("{food}");

    let option = None;
    // let option = Some("Salami");
    let food = option.unwrap_or("Pizza");
    println!("{food}");

    let option: Option<&str> = None;
    let pizza_fan = false;
    let closure = || if pizza_fan { "Pizza" } else { "Hot Pockets" };
    let food = option.unwrap_or_else(closure);
    println!("{food}");
}
