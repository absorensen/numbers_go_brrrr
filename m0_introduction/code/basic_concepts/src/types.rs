pub fn types() {
    // Types can be inferred
    let inferred_int32 = 5;
    let explicit_u32: u32 = 5;

    // Implicit casting is not allowed. 
    // Yay! 
    // Genuinely! It's a major source of bugs.
    // Try changing the '5.0' to '5' and see what happens!
    let my_float: f32 = 5.0;

    // Trying to change the value of 'my_float' is not
    // allowed by the compiler, as the it is declared as immutable.
    // Try commenting in the following line and see how the compiler reacts
    // my_float = 3.2;
    
    // What would be legal would be the following:
    let mut mutable_float: f32 = 2.0;
    // Change mutable_float's value to '6.0'
    mutable_float = mutable_float * 3.0;
    // Change mutable_float's value to '7.2'
    mutable_float += 1.2;



    // Tuples allows you to combine several values in neat little anonymous packages. 
    // Just use () and ,. 
    let tuple: (f32, f32) = (1.0, 5.0);
    let new_tuple: (u32, f32) = (5, 1.0);
    let another_tuple: (f32, u32) = (my_float, explicit_u32);

    // You can even have tuples of tuples!
    let mother_of_all_tuples: ((u32, f32), (f32, u32)) = (new_tuple, another_tuple);
    
    // We can access the fields of a tuple by anonymous access by
    // remembering the order of the tuple

    // Get a '1.0' here
    let first_value: f32 = tuple.0;

    // Get a '5.0' here
    let second_value: f32 = tuple.1;

    // We can even go deeper!
    let more_values: f32 = mother_of_all_tuples.0.1;
    let even_more_values: u32 = mother_of_all_tuples.1.1;

    // Read more about tuples here: https://doc.rust-lang.org/std/primitive.tuple.html



    // One, slightly strange type is the void type.
    // It is written (). It is the default return value of a function
    // if you don't write anything else. It can also be useful with 
    // constructs like Option, where you might just want to denote that
    // your function was successful.
    let a_void_variable: () = ();

    // Option is a construct that allows something to be there or not.
    // If there isn't anything the Option will contain None
    // If there is something, the Option will contain Some containing a type
    // such as Some(u32).
    let option: Option<u32> = None;
    let other_option: Option<u32> = Some(5);

    // Since we can't be sure whether there is Some in the option, we have to check
    // and handle both the consequences. expect() yields a program stopping error (called a panic)
    // with the written message, if other_option contains None.
    let retrieved_value: u32 = other_option.expect("Damnit, this was supposed to be Some(u32)");

    // There is a multitude of ways to handle this, probably in more idiomatic ways,
    // but the quickest ones to wrap your head around are:
    let is_some: bool = option.is_some(); // is false
    let is_none: bool = option.is_none(); // is true

    // Option can also be quite useful as fields of a struct.
    // Say you have a wrapper struct around a big data payload.
    // That struct could have whatever fields you might need around
    // an image, such as the path to the image, 
    // the size of the dimensions, etc., but then have an Option<RawPixels>
    // field for the big payload we may or may not want to carry around.
    // Read more about Option here: https://doc.rust-lang.org/std/option/



    // Result is another widely used construct.
    let good_result: Result<i32, i32> = Ok(10);
    let bad_result: Result<i32, i32> = Err(10);

    // Results have similar functions to is_some and is_none
    let is_ok: bool = good_result.is_ok(); // is true
    let is_error: bool = bad_result.is_err(); // is true

    // Read more about Result here: https://doc.rust-lang.org/std/result/



    // slices


    // Read more about slices here: https://doc.rust-lang.org/std/primitive.slice.html



    // str


    // Read more about str here: https://doc.rust-lang.org/std/primitive.str.html



    // Vec

    // Read more about Vec here: https://doc.rust-lang.org/std/vec/struct.Vec.html



    // String

    // Read more about String here: https://doc.rust-lang.org/std/string/struct.String.html



    // HashMap

    // Read more about HashMap here: https://doc.rust-lang.org/std/collections/struct.HashMap.html

}