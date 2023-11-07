use crate::utility::{GPUHandles, mean_square_error, are_vectors_equivalent};

// The length of filter is assumed to be oddly number, i.e. 1, 3, 5, 7, 9, 11
fn convolution_cpu(signal: &Vec<f32>, filter: &Vec<f32>) -> Vec<f32> {
    let filter_offset = filter.len() / 2;
    let mut output: Vec<f32> = vec![0.0; signal.len()];
    for signal_index in 0..signal.len() {
        for filter_index in 0..filter.len() {
            let offset_signal_index: i64 = signal_index as i64 - filter_offset as i64 + filter_index as i64;
            if -1 < offset_signal_index && offset_signal_index < signal.len() as i64 {
                output[signal_index] += signal[offset_signal_index as usize] * filter[filter_index];
            }
        }
    }

    output
}

fn test_ground_truth() -> bool {
    let signal: Vec<f32> = vec![ 1.0, 1.0, 1.0, 1.0, 1.0];
    let filter: Vec<f32> = vec![ 0.25, 0.5, -0.25];
    let ground_truth_output: Vec<f32> = vec![ 0.25, 0.5, 0.5, 0.5, 0.75];

    let output: Vec<f32> = convolution_cpu(&signal, &filter);

    assert!(output.len() == ground_truth_output.len());
    for index in 0..ground_truth_output.len() {
        if 0.00001 < (output[index] - ground_truth_output[index]).abs() {
            println!("Provided output: {:?}", output);
            println!("Ground truth output: {:?}", ground_truth_output);
            return false;
        }
    }

    true
}

pub fn convolution(handles: &GPUHandles) -> bool {
    // A small test to ensure that the convolution_cpu function is actually correct.
    let ground_truth_is_correct: bool = test_ground_truth();
    println!("Convolution ground truth function is correct: {}", ground_truth_is_correct);
    assert!(ground_truth_is_correct);


    let data_element_count: usize = 1000000;
    let filter_size: usize = 19;
    let signal: Vec<f32> = (0..data_element_count).map(|x| x as f32 * 0.1).collect();
    let filter: Vec<f32> = (0..filter_size).map(|x| x as f32 * 0.1).collect();

    let ground_truth: Vec<f32> = convolution_cpu(&signal, &filter);
    let dummy_data: Vec<f32> = ground_truth.clone();

    //
    // YOUR CODE HERE
    // Make one version of 1D convolution on the GPU which uses if's to guard against the filter
    // Make another version using a zero padded version of the original signal to not use any if's
    // inside the inner for-loop. This zero padding is (filter_size - 1) / 2 on each side of the
    // signal.
    // Both versions should use shared memory.
    // See which is the fastest, is it the signal in shared memory, is it the filter in
    // shared memory, is it both?
    // Make sure to keep the filter and signal large enough to offset the cost of data transfer

    println!("convolution MSE: {}", mean_square_error(&ground_truth, &dummy_data));
    let success: bool = are_vectors_equivalent(&ground_truth, &dummy_data);
    println!("convolution success: {}!", success);

    success
}