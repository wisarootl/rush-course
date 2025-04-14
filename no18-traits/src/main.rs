// mod helper;
// fn main() {
//     // lodging project
//     helper::no01_defining_a_trait::main();
//     helper::no02_a_preview_of_trait_objects::main();
//     helper::no03_trait_must_be_in_scope_to_use_its_definitions::main();
//     helper::no04_moving_to_project_structure::main();

//     // others
//     helper::no05_associated_constants_in_a_trait::main();
//     helper::no06_implementing_the_display_trait_on_a_struct::main();
//     helper::no07_implementing_the_display_trait_on_an_enum::main();
//     helper::no08_implementing_the_debug_trait::main();
//     helper::no09_formatter_methods::main();
//     helper::no10_implementing_the_drop_trait::main();
//     helper::no11_implementing_the_clone_trait::main();
//     helper::no12_implementing_the_copy_trait::main();
//     helper::no13_implementing_the_partial_eq_trait_for_structs::main();
//     helper::no14_defining_equality_for_different_types::main();
//     helper::no15_implementing_the_partial_eq_trait_for_enums::main();
//     helper::no16_implementing_the_eq_trait::main();
//     helper::no17_implementing_the_partial_ord_trait::main();
//     helper::no18_associated_types_i::main();

//     helper::project::main();
// }

use std::fmt::Display;

struct Latte<T> {
    shop: T,
}

impl<T: Display> Latte<T> {
    fn info(&self) {
        println!("The coffee shop is {self:?}");
    }
}
