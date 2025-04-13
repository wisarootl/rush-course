mod helper;
fn main() {
    // lodging project
    helper::no01_defining_a_trait::main();
    helper::no02_a_preview_of_trait_objects::main();
    helper::no03_trait_must_be_in_scope_to_use_its_definitions::main();
    helper::no04_moving_to_project_structure::main();

    // tax project
    helper::no05_associated_constants_in_a_trait::main();

    helper::no06_implementing_the_display_trait_on_a_struct::main();
}
