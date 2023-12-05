use std::time::{Instant, Duration};

fn main() {
    run_access_tests();
}

fn run_access_tests() {
    let iteration_count: usize = 100_000;
    let data_count: i32 = 100;
    run_access_test(iteration_count, data_count);

    let iteration_count: usize = 100_000;
    let data_count: i32 = 1000;
    run_access_test(iteration_count, data_count);

    let iteration_count: usize = 100_000;
    let data_count: i32 = 10_000;
    run_access_test(iteration_count, data_count);

    let iteration_count: usize = 100_000;
    let data_count: i32 = 100_000;
    run_access_test(iteration_count, data_count);
}

fn run_access_test(iteration_count: usize, data_count: i32) {
    let mut data: Vec<i32> = (0..data_count).collect();
    let mut sum: Vec<i32> = vec![0; 1];

    println!("RUNNING ACCESS TESTS WITH {} data elements for {} iterations!", data_count, iteration_count);
    println!("=============================================================");
    
    //
    // Sequential
    //
    println!("Sequential access: {} ms", sequential(&mut data, &mut sum, iteration_count));

    //
    // Non-wrapping strided (actually skipping work)
    //
    let stride: usize = 2;
    println!("Non-wrapping strided access ({}): {} ms", stride, non_wrapping_strided(&mut data, &mut sum, iteration_count, stride));

    let stride: usize = 3;
    println!("Non-wrapping strided access ({}): {} ms", stride, non_wrapping_strided(&mut data, &mut sum, iteration_count, stride));

    let stride: usize = 4;
    println!("Non-wrapping strided access ({}): {} ms", stride, non_wrapping_strided(&mut data, &mut sum, iteration_count, stride));


    //
    // Wrapping strided
    //
    let stride: usize = 1; // And once just to prove that this is just about equal to sequential, despite a bit more work.
    println!("Strided access ({}): {} ms", stride, strided(&mut data, &mut sum, iteration_count, stride));

    let stride: usize = 5; // And once just to prove that this is just about equal to sequential, despite a bit more work.
    println!("Strided access ({}): {} ms", stride, strided(&mut data, &mut sum, iteration_count, stride));

    let stride: usize = 17;// We do this to have the stride be more than the size of a cache line
    println!("Strided access ({}): {} ms", stride, strided(&mut data, &mut sum, iteration_count, stride));


    //
    // Random access
    //
    println!("Random access: {} ms", random(&mut data, &mut sum, iteration_count));

    println!("");
}

fn sequential(data: &mut Vec<i32>, sum: &mut Vec<i32>, iteration_count: usize) -> f64 {
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        sum[0] = 0;
        for index in 0..data.len() {
            sum[0] += data[index];
            data[index] *= 3;
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn non_wrapping_strided(data: &mut Vec<i32>, sum: &mut Vec<i32>, iteration_count: usize, stride: usize) -> f64 {
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        sum[0] = 0;
        let mut index: usize = 0;
        while index < data.len() {
            sum[0] += data[index];
            data[index] *= 3;
            index += stride;
        }
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn strided(data: &mut Vec<i32>, sum: &mut Vec<i32>, iteration_count: usize, stride: usize) -> f64 {
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        sum[0] = 0;
        for index in 0..data.len() {
            let index: usize = (index * stride) % data.len(); 
            sum[0] += data[index];
            data[index] *= 3;
        }    
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

#[inline(always)]
fn randomish_hash(index: usize, limit: usize) -> usize {
    (index * 37199 + 999331) % limit
}

fn random(data: &mut Vec<i32>, sum: &mut Vec<i32>, iteration_count: usize) -> f64 {
    let data_length: usize = data.len();
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        sum[0] = 0;
        for index in 0..data_length {
            let index: usize = randomish_hash(index, data_length); 
            sum[0] += data[index];
            data[index] *= 3;
        }    
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}