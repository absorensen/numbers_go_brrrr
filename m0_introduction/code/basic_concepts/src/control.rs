pub fn control () {
    // The control statements that we will see in the guide
    // are the basic ones. Note that Rust prefers no
    // parenthesis around the condition for control
    // statements.

    // The simplest one is loop. It is equivalent to
    // while(true) in other languages. It is broken out
    // of by returning from the function or a break 
    // statement.

    let mut counter: usize = 0;
    loop {
        counter += 1;

        if 5 < counter {
            break;
        }
    }

    // Of course there is also the classic while loop.
    let mut counter: usize = 0;
    while counter < 5 {
        counter += 1;
    }

    // The if-statement is a bit different in Rust.
    // It returns a value.

    let some_value: u32 = 42;
    let which_case: u32 = 
        if some_value == 5 {
            5
        } else if some_value == 7 {
            7
        } else {
            42
        // Note the semicolon here
        };

    // Although, in this case we might as well have written
    let which_case: u32 = match some_value {
        5 => 5,
        7 => 7,
        _ => 42,
    };

    // There is no ternary operator: condition ? 42 : 41;
    // What you can do is this
    let some_value: u32 = 42;
    let is_42: bool = if some_value == 42 { true } else { false };

    // Rust does not have the classic C/C++ for-loop:
    // for(int i = 0; i < vector.size(); ++i)
    // instead it has range based for-loops and iterators.
    let mut our_vector: Vec<u32> = vec![1, 2, 3, 4, 5];

    // Note the reference in front of our_vector
    // otherwise the for-loop takes ownership of
    // our_vector and effectively drains the values.
    for value in &our_vector {
        // values will be &1, &2, &3, &4, &5
    } 

    // We can also do this with mutable references
    for mutable_value in &mut our_vector {
        // Remember to dereference to get to the 
        // underlying value
        *mutable_value *= 2;
    }

    // Another possibility would be
    for index in 0..our_vector.len() {
        // index will be 0, 1, 2, 3, 4
        // Values will be 2, 4, 6, 8, 10
        our_vector[index];
    }

    // You can also implement basically the same functionality,
    // but it becomes a lot like implementing a for-loop
    // with a while loop.
    let mut index = 0;
    while index < our_vector.len() {
        // All of the same indices and values as the loops above.
        our_vector[index];

        index += 1;
    }

}