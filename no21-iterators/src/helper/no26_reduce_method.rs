pub fn main() {
    println!("=== no26_reduce_method ===");
    let earnings = [4, 7, 9, 13];

    let sum = earnings
        .into_iter()
        .reduce(|total, current| total + current);
    println!("{sum:?}"); // reduce use 4 as starting value

    let address_portions = [
        String::from("123 Elm Street"),
        String::from("Suburbia"),
        String::from("New Jersey"),
    ];

    println!("{}", address_portions.join(", "));

    let address = address_portions
        .into_iter()
        .reduce(|mut accumulator, portion| {
            accumulator.push_str(", ");
            accumulator.push_str(&portion);
            accumulator
        });

    println!("{address:?}");
}
