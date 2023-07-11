// Allow unused variables and code. 
// We are just showing examples here.
#![allow(
    dead_code, 
    unused_variables, 
    unused_assignments,
    unused_must_use
)]

// Include the other files
mod more_iterators;
use crate::more_iterators::more_iterators;

mod smart_pointers;
use crate::smart_pointers::smart_pointers;

mod traits;
use crate::traits::traits;

mod generics;
use crate::generics::generics;

mod gentle_lifetimes;
use crate::gentle_lifetimes::gentle_lifetimes;

mod closures;
use crate::closures::closures;

fn main() {
    // Iterators
    more_iterators();

    // Smart pointers
    smart_pointers();

    // Traits
    traits();

    // Generics
    generics();

    // Closures
    closures();

    // Gentle Lifetimes
    gentle_lifetimes();

}
