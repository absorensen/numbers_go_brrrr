pub fn iterators() {
    // The cool thing about iterators is how it allows the 
    // compiler certainty in how the elements are accessed. 
    // This can sometimes result in quite a performance boost.
    // If you run a classic for-loop with indices, 
    // the compiler has no guarantees that you
    // only access each element once. You could be using a
    // hash function to generate a semi-random index or
    // override the index to only choose the index 5.

    // We start with the range statement (0..1000),
    // which outputs all values between 0 and 1000.
    // We turn that into a Vec<u32>
    // by writing collect() - meaning 'Hey! Execute this statement".
    // This is needed because iterators are lazy by default.
    // It needs to be told when to execute.
    // Comment this back in to see what compiler error
    // this will result in.
    // let data = (0..1000).collect();

    // An important thing to know is, if the collect() function
    // can't figure out what sort of data structure and types
    // it should "collect", it needs a type to output to.
    let data: Vec<u32> = (0..1000).collect();

    // Iterators are lazy and can both be chained and stored.
    // Note that none of these variables need to be mutable
    // as each of these manipulations result in new data.
    // After this section, we'll see how to manipulate these
    // values in place.
    // This is one of the places where it makes quite good sense
    // not to strictly define the type of the variable.
    // If you are in an IDE, hover your mouse over
    // my_iterator_im_saving_for_later and spend at least 30 
    // seconds trying to understand the exact type.
    // You might need to learn about lifetimes, traits and Fn first.
    let my_iterator_im_saving_for_later = 
        // Get an iterator over the vector
        data.iter() 
        // Perform map, an operation on every 
        // element of the collection (not 
        // necesarrily vector), which here is an 
        // anonymous function (closure). Note the lack 
        // of semicolon, meaning that we return double
        // of the new value.
        .map(|x: &u32| *x * 2)
        // Here we taking in the value returned by map.
        // Thus we should be getting the values 0..2000.
        // Filter returns only the values which yielded
        // a true value. In this case we should be end up
        // with a list of values 0, 2, 4, 6 .. 498. 
        .filter(|x: &u32| *x < 500);

    // Actually execute the iterators with the collect(). 
    // Until now we have only been assembling a recipe. We won't
    // be paying for most of the process until this statement.
    // This is handy because we can assemble however many iterators 
    // we need and then do other stuff with them, send them somewhere,
    // space them out to not drain the battery of our device... whatever
    // we need!
    let manipulated_data: Vec<u32> = my_iterator_im_saving_for_later.collect();
    
    // A couple of other basic iterators are sum, min, max, zip 
    // and sort (not actually an iterator, but very useful).
    let sum_of_all_numbers: u32 = manipulated_data.iter().sum();
    let min_of_all_numbers: u32 = *(manipulated_data.iter().min().expect("Overflowed"));
    let max_of_all_numbers: u32 = *(manipulated_data.iter().max().expect("Overflowed"));
    let mut filtered: Vec<u32> = 
        manipulated_data.iter()
        .filter_map(|x: &u32| if 42 < *x { Some(1000 - *x) } else { None } )
        .collect();

    // Now filtered is sorted!
    filtered.sort();

    // Another thing we could do, is to use the zip iterator to combine two collections
    // in a one-for-me-one-for-you fashion.
    let other_filtered: Vec<u32> = filtered.iter().map(|x: &u32| *x * 4).collect();

    // Instead of putting the type annotation on the variable declaration,
    // we can also use the turbofish ::<> anotation on collect to 
    // make collect choose the right type to output.
    // into_iter is also a consuming iterator. Filtered is not available
    // for use after using into_iter(). In general, into means consuming.
    let zipped = filtered.iter().zip(other_filtered.iter()).map(|(a, b)| (*a, *b)).collect::<Vec<(u32, u32)>>();

    // let zipped_filtered = filtered.into_iter().zip(other_filtered.iter()).collect::<Vec<u32>>();
}