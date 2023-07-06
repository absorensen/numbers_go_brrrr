// Allow unused variables and code. 
// We are just showing examples here.
#![allow(
        dead_code, 
        unused_variables, 
        unused_assignments
    )]

// Include the other files
mod types;
use crate::types::types;

mod function_definitions;
use crate::function_definitions::function_definitions;

mod borrow_checker;
use crate::borrow_checker::borrow_checker;

mod shared_and_mutable_references;
use crate::shared_and_mutable_references::shared_and_mutable_references;

mod move_copy_and_clone;
use crate::move_copy_and_clone::move_copy_and_clone;

mod structs;
use crate::structs::structs;

mod enums_and_match;
use crate::enums_and_match::enums_and_match;

mod control;
use crate::control::control;

mod iterators;
use crate::iterators::iterators;

mod errors;
use crate::errors::errors;

fn main() {
    println!("Hello, basic concepts in Rust!");

    types();
    function_definitions();
    borrow_checker();
    shared_and_mutable_references();
    move_copy_and_clone();
    structs();
    enums_and_match();
    control();
    iterators();
    errors();
}
