use std::collections::HashMap;

// Now that we have been through references
// we have the basic components to talk about
// more advanced types.
pub fn more_types() {
    // slices are references to multiple elements
    // An example could be the first two elements 
    // of a four element array.
    // Don't worry, we will get into what a vec
    // is in a second. Just think array!
    
    // We intialize a Vec<i32> with the vec! macro.
    // A macro is something that happens before the code is compiled.
    // The macro takes the vec! area of the code and then substitutes
    // some other code. What the compiler actually sees is the new code.
    // Macros in Rust are suffixed with !. This also means that 
    // the print statement you might have seen earlier - println!()
    // is a macro.
    // We create a vector
    let vec: Vec<i32> = vec![1, 2, 3];
    // We get a slice to all of the values in vec.
    // Note that .. is a range.
    // In this context it basically means "all of the values"
    let int_slice: &[i32] = &vec[..];

    // The range statement is inclusive of the first value,
    // but not the second value.
    // This slice borrows the first 2 values [1, 2]
    let other_int_slice: &[i32] = &vec[0..2];
    
    // We can of course also get mutable slices.
    let mut original_values: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4];
    {
        // Get references to [0.3, 0.4]
        let right_half: &mut [f32] = &mut original_values[2..4];

        // We aren't allowed to do this 
        // let left_half: &mut [f32] = &mut original_values[0..2];
        // as the compiler can't completely
        // guarantee that there are no overlaps between the two slices.
        // It is however possible to do this using something called iterators
        // and the chunk() which guarantees non-overlapping slices.
        // let chunk_size: usize = 2;
        // let mutable_slices: Vec<&mut [f32]> = original_values.chunks_mut(chunk_size).collect();
        // The guide will in general try to avoid this kind of thing
        // to not overwhelm you, but there are some more techniques like this
        // explained at levels 3 and 4. Nudge nudge.

        // Note the change in indexing. Index 0 for this slice corresponds
        // to index 2 in original_values. The resulting value being 0.6.
        right_half[0] *= 2.0;

    }
    // Read more about slices here: https://doc.rust-lang.org/std/primitive.slice.html



    // str
    // A str is known as a string slice. str is always UTF-8 encoded. 
    // Quite a lot of the time you will see the compiler say &'static str. 
    // This boils down to being a reference to
    // a string (array of characters) which will live for the duration
    // of the program, that's the 'static part. Usually the ' will refer
    // to the lifetime (how long it is valid) of a variable. The guide
    // will try to save you from the world of lifetimes. It will be glanced
    // at at level 3, but that is it. We promise!

    let some_str: &str = "my_str";
    
    // You can't index a str like you can a normal slice!
    // No way compadre!
    // let fourth_character = some_str[3];
    let fourth_character: Option<char> = some_str.chars().nth(3);
    let fourth_character: char = 
        fourth_character
            .expect(
                // Also a &'static str, by the way!
                "I was sure I correctly asked for the fourth character, but somethin went horribly wrong!"
            );

    // Read more about str here: https://doc.rust-lang.org/std/primitive.str.html



    // The Vec<T> type is a vector, which is a resizeable array. The <>
    // symbols denote a generic. Basically, Vec<T>, means a vector
    // which carries the type T. This could be Vec<u32>, Vec<f32> or
    // even Vec<SomeRandomTypeIMade>. The generic means that there
    // doesn't need to be a bunch of code written that does basically
    // the same thing. Generics will be showcased a bit more at levels 3 and 4.
    let mut a_vector: Vec<u32> = Vec::<u32>::new();
    a_vector.push(0);
    a_vector.push(1);
    a_vector.push(2);
    let a_vector: Vec<u32> = a_vector;
    // The above could be accomplished by the following
    let a_vector: Vec<u32> = vec![0, 1, 2];
    // We can also allocate N elements with some default value
    let a_vector: Vec<u32> = vec![2; 3]; // Same as [2, 2, 2]


    // Vec is the most used collection, 
    // it is highly recommended that you read more about
    // Vec here: https://doc.rust-lang.org/std/vec/struct.Vec.html



    // A String is the Vec version of a str. It is resizable, it can contain
    // dynamic run-time defined content, you can add them together.
    let mut my_string: String = String::from("Hello");
    my_string.push(' '); //Just pushing a char here
    my_string.push_str("There!");
    assert_eq!("Hello There!", my_string);
    
    // You can get a str reference to the String
    let hello_there: &str = my_string.as_str();

    // And turn a str into a String
    let hello_there: String = hello_there.to_string();

    // The underlying data of String is a [char], but since it, like str,
    // upholds UTF-8, you cannot index directly, as some characters might be
    // big enough to take up more than element in the array. Like this 
    // Sparkle Heart from the link given below:

    let sparkle_heart: Vec<u8> = vec![240, 159, 146, 150];
    let sparkle_heart:String = String::from_utf8(sparkle_heart).unwrap();

    // SPARKLE
    assert_eq!("💖", sparkle_heart);
    // HEART

    // Read more about String here: https://doc.rust-lang.org/std/string/struct.String.html



    // HashMap<K, V> is sometimes called a dictionary in other languages. 
    // Inserting some Key of type K, maps to
    // some Value of type V. This could be
    let mut donuts_eaten_count: HashMap<String, u32> = HashMap::new(); 
    donuts_eaten_count.insert("Dave".to_string(), 0);
    donuts_eaten_count.insert("Stanley".to_string(), 0);

    // or
    let mut donuts_eaten_count: HashMap<String, u32> = 
        HashMap::from([
            ("Dave".to_string(), 0), 
            ("Stanley".to_string(), 0) 
            ]);

    let donuts: &mut u32 = donuts_eaten_count.entry("Dave".to_string()).or_insert(1);
    *donuts += 10;

    donuts_eaten_count
        .entry("Stanley".to_string())
        .and_modify(|eaten_count| *eaten_count += 2001)
        .or_insert(1);

    // In general the HashMap can be a little bit more finicky to 
    // work with compared to a Vector as values may or may not be there. 
    // Read more about HashMap here: https://doc.rust-lang.org/std/collections/struct.HashMap.html
}