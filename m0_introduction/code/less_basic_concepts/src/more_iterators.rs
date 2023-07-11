pub fn more_iterators() {
    // Often, when using the for-each loop we might be missing knowing the
    // relevant index in the data structure. For this there is the 
    // enumerate adaptor.
    let data: Vec<u32> = vec![01, 12, 23];
    for (index, element) in data.iter().enumerate() {
        // Should print 0, 1; 1, 12; 2; 23.
        println!("index: {}, element: {}", index, *element);
    }

    // This can be very useful if writting to a given output.
    let mut output_data: Vec<u32> = vec![0, 0, 0, 0, 0];
    for (index, element) in data.iter().enumerate() {
        output_data[index] = *element;
    }

    // Another way of doing this could be through zip!
    for(input_element, output_element) in data.iter().zip(output_data.iter_mut()){
        *output_element = 24 + *input_element;
    }
    
    // If you want to perform a reduce operation, that is not using the '+' operator,
    // in which you should use the .sum() function, you can use the .fold() adapter.
    // It takes an initial value for an accumulator and a closure for processing
    // the accumulator and the current elements value.
    let quadruple_sum: u32 = 
        data.iter()
        .fold(0, |accu, element| accu + *element * 4);

    // The reduce operator does almost the same thing, but it skips the first element
    // and the initial value is the value of the first element.
    let sum: u32 = data.into_iter().reduce(|accu, element| accu + element).unwrap();

    // The adapter count() lets us know how many elements are left in the iterator.
    // This will return 
    let data: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6]; // length 7!

    // element_count == 7
    let element_count: usize = data.iter().count();

    // The take adaptor yiels a new operator of the first
    // n elements of the iterator you call it on.
    // So for this chain we first have an iterator of 
    // 7 elements, then 5 elements [0, 1, 2, 3, 4],
    // and then 2 elements [0, 1]. Finally,
    // element_count == 2
    let element_count: usize = data.iter().take(5).take(2).count();

}