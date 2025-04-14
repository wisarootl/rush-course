mod helper;

fn main() {
    helper::no01_concrete_lifetimes_for_values::main();
    helper::no02_concrete_lifetimes_for_references::main();
    helper::no03_non_lexical_lifetimes::main();
    helper::no04_invalid_lifetimes::main();
    helper::no05_functions_cannot_return_references_to_owned_values_or_parameters::main();
    helper::no06_references_as_function_parameters::main();
    helper::no07_intro_to_generic_lifetimes::main();
    helper::no08_lifetimes_and_referents::main();
    helper::no09_lifetime_elision_rules_i::main();
    helper::no10_multiple_parameters::main();
    helper::no11_lifetime_elision_rules_ii::main();
}
