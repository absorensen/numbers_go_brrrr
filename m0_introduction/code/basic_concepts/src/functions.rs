// This section will be quite small
// You have already seen functions,
// the guide will just show you a few
// interesting tidbits.
pub fn functions() {
    let mut inlined_result: Option<(f32, u64)> = my_inlined_function(2);
    let mut void: () = returning_void_implicitly();
    void = returning_void_explicitly();
    inlined_result = function_taking_functions(5, my_inlined_function);

    let returning_early_result: Result<i64, &str> = returning_early(5, 3.2, "A str"); 
}

// We can add annotations to our functions.
// This one asks the compiler to basically copy/paste
// this function into anywhere it is called.
// This removes the overhead of calling a function for what
// might otherwise be very simple code. Inlining can be 
// a bit finicky, and you should probably measure what the
// impact is, unless the function is very simple computationally.
// Stuff like accessing a variable or adding two numbers.
#[inline(always)]
fn my_inlined_function(argument_a: u32) -> Option<(f32, u64)> {
    Some((3.1, argument_a as u64 * 2))
}

// We don't have to write a return type...
fn returning_void_implicitly() {

}

// It will be the same as writing the return as below
fn returning_void_explicitly() -> () {

}

// Uuuuuh, we can even take a function as input.
// This will be used quite a bit to benchmark a lot
// of similar functions in the memory_hierarchies 
// code framework.
fn function_taking_functions(
    argument_a: u32, 
    function_input: fn(u32)->Option<(f32, u64)>
) -> Option<(f32, u64)> {
    function_input(argument_a)
}

fn returning_early(int: u32, float: f64, message: &str) -> Result<i64, &str> {
    // If works slightly different
    // in Rust, more on that later.
    if int < 1 {
        if 5.0 < float {
            // Early return statement.
            return Err("Nah, returning early");
        }

        // This can never be true
        if 10 < int {
            // Early return statement.
            return Err("That int was way too big!");
        }

    }

    // Return statement.
    // Note the lack of ;
    Ok(2)
}
