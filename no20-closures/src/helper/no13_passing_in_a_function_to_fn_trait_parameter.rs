#![allow(unused)]

fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(),
{
    procedure();
    procedure();
    procedure();
}

fn bake_cake() {
    println!("Hello chocolate cake");
}

pub fn main() {
    println!("=== no13_passing_in_a_function_to_fn_trait_parameter ===");
    let option: Option<Vec<String>> = None;
    let collection = option.unwrap_or_else(Vec::new);
    println!("{:?}", collection);
}
