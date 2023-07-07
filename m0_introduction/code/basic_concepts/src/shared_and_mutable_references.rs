pub fn shared_and_mutable_references() {
    // References are basically an address to a value.

    let central_value: u32 = 5;

    // These references without any mut keywords
    // are called shared references.
    // You can make as many of these as you like.
    let reference_one: &u32 = &central_value;
    let reference_two: &u32 = &central_value;
    let reference_three: &u32 = &central_value;

    // You can follow the address to get to the value.
    let sum: u32 = *reference_one + *reference_two + *reference_three;
    
    // But not all references are created equal.
    // Using references allows us to access a variable
    // without moving or cloning it. What we can't do
    // is have multiple mutable references to the same
    // variable. This is an absolutely central part of
    // the Rust way of doing things.¨
    let mut another_value: u32 = 5;
    
    // This mutable reference borrows the ownership
    // of the value in another_value. It will be returned
    // to another_value once the mutable_reference variable
    // is dropped (Rusts word for destroyed/removed).
    let mutable_reference: &mut u32 = &mut another_value;
    
    // Accessing another_value is not allowed
    // another_value += 1;

    *mutable_reference += 1;
    assert!(another_value == 6);

    // We can make another mutable reference to another_value.
    // But we can no longer access the old reference
    let another_mutable_reference: &mut u32 = &mut another_value;
    *another_mutable_reference += 1;

    // If this is uncommented 
    // *mutable_reference += 1;
    // the compiler will complain about
    // assert!(another_value == 6) as borrowing 
    // and let another_mutable_reference: &mut u32 = &mut another_value;
    // as a second mutable borrow. As it is now, the compiler probably
    // sees that mutable_reference is no longer needed at the line
    // assert!(another_value == 6);
    // returns the mutable borrowed ownership to another_value
    // then creates another mutable borrow of another_value
    // to make another_mutable_reference.

    // The mechanism keeping track of which references go where and what is
    // allowed is called THE BORROW CHECKER. Not always in all caps, but it
    // will usually what you will find people complaining about on 
    // Rust related StackOverflow questions. The thing is... the borrow checker
    // (see no caps), is actually your friend. Your really annoying friend
    // keeping you from making mistakes. If the borrow checker says your code
    // is ok, it will usually be relatively easy to parallelize. It is what
    // Rustaceans (the people who self identify as Rust fans), call
    // FEARLESS CONCURRENCY.
    
    // Another thing to note is that if a mutable reference is held anywhere,
    // no shared references are allowed.
    let mut some_value: f32 = 3.15;
    let mutable_reference: &mut f32 = &mut some_value;
    *mutable_reference *= 1.0;

    // Now illegal! Try uncommenting the first line and see what the compiler says!
    // let shared_referece: &f32 = &some_value;

    // Then uncomment this second line!
    // let the_value: f32 = shared_referece.clone();

    // As it turns out, the compiler has gotten you covered, right until it can't
    // just drop references you no longer need.

    // One way to solve this, is to move this line below
    // *mutable_reference *= 1.0;
    *mutable_reference -= 2.1;

    // We can also be more explicit about when we want to drop a variable.
    // One way is brackets. This creates a scope. Everything declared
    // inside the scope is dropped once the scope ends.

    // Variable has to be declared mut for us to mutably borrow
    // let original_owner: f32 = 5.321;
    let mut original_owner: f32 = 5.321;

    {
        let mutable_original_reference: &mut f32 = &mut original_owner;
        *mutable_original_reference += 2.0;
        *mutable_original_reference += 3.0;
    } 

    // Illegal operation. mutable_original_reference is no longer a valid name!
    //*mutable_original_reference += 1.0;

    // Legal again!
    // The mutable borrow from mutable_original_reference has been dropped 
    // and ownership has been returned to original_owner.
    original_owner += 1.4;

    {
        let another_mutable_original_reference: &mut f32 = &mut original_owner;
        *another_mutable_original_reference += 2.0;
        *another_mutable_original_reference += 3.0;
    }

    // Legal once again!
    // The mutable borrow has been dropped and returned ownership to
    // original_owner.
    original_owner += 1.1;


    // References are often used for function arguments
    let unsigned: u32 = 2;
    let signed: i32 = -1;

    // Not allowed getting a mutable reference from this
    //let float: f32 = 1.0;
    let mut float: f32 = 1.0;

    // Perfectly legal
    let function_went_well: bool = 
        reference_function(
            &unsigned, 
            &signed, 
            &mut float
        );

    // Highly illegal to have more than 1 mutable reference
    // to any variable.
    // let function_went_well: bool =
    //     multiple_mutable_references(
    //         &unsigned, 
    //         &signed, 
    //         &mut float, 
    //         &mut float
    //     );

    // A more legal way of doing things
    let mut other_float: f32 = float.clone();
    let function_went_well: bool =
        multiple_mutable_references(
            &unsigned, 
            &signed, 
            &mut float, 
            &mut other_float
        );

    float += other_float;

    // We could accomplish having several mutable references,
    // in a way, through some workarounds. This is a more advanced
    // topic and won't be introduced until module 2, the one about
    // concepts in parallelism. It is also more of a level 3 and 4
    // kind of concept. Anyways, if you find yourself needing to worry
    // about this sort of thing, you are either overcomplicating 
    // your code, or you should be taking things to the next level 
    // (3 and 4).

}

fn reference_function(
    argument_a: &u32, 
    argument_b: &i32, 
    // A mutable reference, especially several mutable references
    // can be very useful for changing more than 1 variable
    // it also means that the original value does not have to move
    // or change hands. An alternative would be to repack
    // into structs. Maybe even return a collection of changed variables
    // which could then overwrite the previous values.
    // This can however result in a bit of moves and clones.
    argument_c: &mut f32
) -> bool {
    // Not allowed
    //argument_a += 2;

    // You need to dereference
    // the reference to mutate it.
    // This is not needed for function calls
    // such as .abs()
    *argument_c += 2.0 * argument_c.abs();

    // Twas a success
    true
}

fn multiple_mutable_references(
    argument_a: &u32, 
    argument_b: &i32, 
    argument_c: &mut f32, 
    argument_d: &mut f32
) -> bool {
    *argument_c += 2.1;
    *argument_d -= argument_c.floor();

    // Twas not a success
    false
}