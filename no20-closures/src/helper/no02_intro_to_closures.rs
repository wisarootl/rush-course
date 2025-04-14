pub fn main() {
    println!("=== no02_intro_to_closures ===");
    let multiplier = 3;

    let multiply_by = |value: i32| -> i32 {
        return value * multiplier;
    };
    println!("{}", multiply_by(5));

    let product = |a: i32, b: i32| -> i32 {
        println!("Calculating product for you");
        return a * b;
    };
    println!("{}", product(3, 10));
    println!("{}", product(5, 8));
}
