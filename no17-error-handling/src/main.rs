mod helper;

fn main() {
    // helper::no01_panic_macro::main(); // panic
    // helper::no02_process_module_and_the_exit_function::main(); // exit 1
    helper::no03_standard_error::main();
    // helper::no04_opening_a_file::main(); // exit 1
    // helper::no05_asking_the_user_for_input::main(); // require user input
    // helper::no06_reading_the_files_contents::main(); // require user input
    // helper::no07_propagating_errors::main(); // require user input
    // helper::no08_question_mark_operator::main(); // require user input
    // helper::no09_the_read_to_string_associated_function::main(); // require user input
    helper::no10_using_question_mark_with_option::main();
    helper::project::main(); // require user input
}
