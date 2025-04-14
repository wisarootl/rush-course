fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(),
{
    procedure();
    procedure();
    procedure();
}

pub fn main() {
    println!("=== no12_fn_trait ===");
    let mut bosses = vec!["Boris"];
    let closure = || {
        bosses.push("Alexandra");
    };
    execute_thrice(closure);
}
