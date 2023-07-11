pub fn iterators() {
    // The cool thing about iterators is how it allows the 
    // compiler certainty in how the elements are accessed.
    // If you run a classic for-loop with indices, 
    // the compiler has no guarantees that you
    // only access each element once. You could be using a
    // hash function to generate a semi-random index or
    // override the index to only choose the index 5.
    // Generally, you can assume that loop and iterator
    // strategies have comparable performance.
    // But there are some cases where iterators may be faster.

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
        // necessarily vector), which here is an 
        // anonymous function (closure). Note the lack 
        // of semicolon, meaning that we return a new value
        // which is double the old value
        .map(|x: &u32| *x * 2)
        // Here we taking in the value returned by map.
        // Thus we should be getting the values 0..2000.
        // Filter returns only the values which yielded
        // a true value. In this case we should be end up
        // with a list of values 0, 2, 4, 6 .. 498. 
        .filter(|x: &u32| *x < 500);

 
    // Until now we have only been assembling a recipe. We won't
    // be paying for most of the process until this statement.
    // This is handy because we can assemble however many iterators 
    // we need and then do other stuff with them, send them somewhere,
    // space them out to not drain the battery of our device... whatever
    // we need!

    // Actually execute the iterators and iterator adaptors (map and filter) 
    // with the collect().
    let manipulated_data: Vec<u32> = my_iterator_im_saving_for_later.collect();
    
    // A couple of other basic iterator adaptors are sum, min, max, zip 
    // and sort (not actually an iterator adapter, it's a function on Vec,
    // but it's very useful).
    let sum_of_all_numbers: u32 = manipulated_data.iter().sum();
    let min_of_all_numbers: u32 = *(manipulated_data.iter().min().expect("Overflowed"));
    let max_of_all_numbers: u32 = *(manipulated_data.iter().max().expect("Overflowed"));
    let mut filtered: Vec<u32> = 
        // Get an iterator of shared references to our underlying data
        manipulated_data.iter()
        // filter_map is a combination of filter() and map() chained.
        // An element is part of our output elements if the new value is 
        // wrapped in Some() (an Option<u32> in this case). The value we 
        // get in our collection however is the unwrapped Option, so just
        // u32.
        .filter_map(|x: &u32| if 42 < *x { Some(1000 - *x) } else { None } )
        .collect();

    // Now filtered is sorted!
    filtered.sort();

    // Another thing we could do, is to use the zip iterator to combine two collections
    // in a one-for-me-one-for-you fashion. If vector A has length 3 and vector B has
    // length 9, we would get a new vector AB* with elements [A0, B0, A1, B1, A2, B2].
    // The rest of vector B is ignored.
    let other_filtered: Vec<u32> = filtered.iter().map(|x: &u32| *x * 4).collect();

    // Instead of putting the type annotation on the variable declaration,
    // we can also use the turbofish ::<> annotation on collect to 
    // make collect choose the right type to output.
    // into_iter is also a consuming iterator. filtered is not available
    // for use after using into_iter(). Neither is other_filtered.
    // In general, into means consuming. If we had used .iter() instead
    // it would be the same as 'for element in &filtered'. The element
    // we would be getting would only be shared references and we would
    // have to do additional manipulation (a map operation), to get 
    // non-references to put into our new vector.
    let zipped = 
        // Turn our first vector into an iterator
        filtered.into_iter()
        // zip the two iterators
        .zip(other_filtered.into_iter())
        .collect::<Vec<(u32, u32)>>();

    // Read more about iterators here: https://doc.rust-lang.org/book/ch13-02-iterators.html

}