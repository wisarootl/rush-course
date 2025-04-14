pub fn main() {
    println!("=== no03_closure_shortcuts ===");
    let multiplier = 5;

    let multiply_by = |value| value * multiplier;
    println!("{}", multiply_by(3 as u8)); // infer type backward from first implementation

    let mirror = |value| value;
    println!("{}", mirror("Why"));

    // Invalid due to inference
    // println!("{}", mirror("Why"));
}
