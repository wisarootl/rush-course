mod helper;

fn main() {
    helper::no01_nested_functions::main();
    helper::no02_intro_to_closures::main();
    helper::no03_closure_shortcuts::main();
    helper::no04_closures_that_capture_immutable_references::main();
    helper::no05_closures_that_capture_mutable_references::main();
    helper::no06_closures_with_ownership::main();
    helper::no07_move_keyword::main();
    helper::no08_unwrap_or_else_method::main();
    // helper::no09_defining_a_method_that_accepts_a_closure_i::main(); // require user input
    helper::no10_string_retain_method::main();
    helper::no11_defining_a_method_that_accepts_a_closure_ii::main();
    helper::no12_fn_trait::main();
    helper::no13_passing_in_a_function_to_fn_trait_parameter::main();
    helper::project::main();
}
