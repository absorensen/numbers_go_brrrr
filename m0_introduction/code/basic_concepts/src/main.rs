// Allow unused variables and code. 
// We are just showing examples here.
#![allow(
        dead_code, 
        unused_variables, 
        unused_assignments,
        unused_must_use
    )]

// Include the other files
mod types;
use crate::types::types;

mod functions;
use crate::functions::functions;

mod move_clone_and_copy;
use crate::move_clone_and_copy::move_clone_and_copy;

mod shared_and_mutable_references;
use crate::shared_and_mutable_references::shared_and_mutable_references;

mod more_types;
use crate::more_types::more_types;

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
    functions();
    move_clone_and_copy();
    shared_and_mutable_references();
    more_types();
    structs();
    enums_and_match();
    control();
    iterators();
    errors();

}
