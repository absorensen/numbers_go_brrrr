// In this section move, clone and copy will
// be introduced. They are very small concepts,
// but they are pervasive.
pub fn move_clone_and_copy() {
    // Let's start with move
    // We assign a value to a variable
    let first_variable: Vec<u32> = vec![0; 5];

    // In most languages second_variable would now
    // contain an array with 5 instances of 0.
    // It is the same in Rust.
    // What is different in Rust, is that
    // first_variable is now no longer accesible.
    let second_variable: Vec<u32> = first_variable;
    
    // This will result in a harsh rebuke from
    // the Rust compiler.
    // println!("{:?}", first_variable);

    // That's right, that's not just a runtime error,
    // the compiler will check and refuse to run the
    // code. It keeps track of whether one variables
    // values has been handed over to another variable.
    
    // One thing we could do is to make a full copy of the
    // data by cloning it!
    // Now we have two values!
    let third_variable: Vec<u32> = second_variable.clone();
    println!("{:?}", second_variable);
    println!("{:?}", third_variable);

    // Aggresively using clone everywhere can be one way
    // to get your code past the compiler intially,
    // eventually, you might find that your code is running
    // slower than expected. One of the first places to look
    // is for excessive cloning. Especially for large data
    // amounts and/or in a loop. Cloning is always an explicit
    // function call, you make, making it easy to spot.

    // Copy, on the other hand is implicit.
    // There is something called traits, don't worry about it,
    // it will be introduced at levels 3 and 4. One of the 
    // major traits is Copy. Various types can implement 
    // the Copy. A type can implement several traits.
    // Basically, Copy is a simple bit-for-bit copy of a
    // value. Most simple types, such as u32, implement it
    // by default.
    let variable: u32 = 5;
    let changed_u32: u32 = copy_the_argument(variable);
    println!("{}", variable);
    println!("{}", changed_u32);

    // Note that we can still access third_variable.
    // It wasn't moved, it was copied!
    // We can do the same with clone!
    let another_changed_u32: u32 = copy_the_argument(variable.clone());
    println!("{}", variable);
    println!("{}", another_changed_u32);


}

// The argument may be copied, but the returned value
// is moved.
fn copy_the_argument(mut a_copied_value: u32) -> u32 {
    a_copied_value += 42;

    // Move that value
    a_copied_value
}