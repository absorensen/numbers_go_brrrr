use std::time::{Instant, Duration};

fn main() {
    run_access_test();
}

fn run_access_test() {
    let iteration_count: usize = 1000;
    let data_count: usize = 16;
    let mut dummy_sum: i32 = 0; // Rust kept optimizing the function calls away in release mode.

    println!("RUNNING ACCESS TESTS WITH 16x16x16 data elements for {} iterations!", iteration_count);
    println!("=============================================================");
    
    //
    // Multi-Array
    //
    println!("Multi-Array Row-Major access: {} ms", multi_array_16_row_major(iteration_count, &mut dummy_sum));
    println!("Multi-Array Column-Major access: {} ms", multi_array_16_column_major(iteration_count, &mut dummy_sum));

    //
    // Multi-Vec
    //
    println!("Multi-Vec Row-Major access: {} ms", multi_vec_row_major(data_count, iteration_count, &mut dummy_sum));
    println!("Multi-Vec Column-Major access: {} ms", multi_vec_column_major(data_count, iteration_count, &mut dummy_sum));

    //
    // Single Vec
    //
    println!("Vec Row-Major access: {} ms", vec_row_major(data_count, iteration_count, &mut dummy_sum));
    println!("Vec Column-Major access: {} ms", vec_column_major(data_count, iteration_count, &mut dummy_sum));

    //
    // Element-Wise Vec
    //
    println!("Vec Element-Wise access: {} ms", vec_elementwise(data_count, iteration_count, &mut dummy_sum));

    println!("");


    let data_count: usize = 32;
    println!("RUNNING ACCESS TESTS WITH 32x32x32 data elements for {} iterations!", iteration_count);
    println!("=============================================================");
    
    //
    // Multi-Array
    //
    println!("Multi-Array Row-Major access: {} ms", multi_array_32_row_major(iteration_count, &mut dummy_sum));
    println!("Multi-Array Column-Major access: {} ms", multi_array_32_column_major(iteration_count, &mut dummy_sum));

    //
    // Multi-Vec
    //
    println!("Multi-Vec Row-Major access: {} ms", multi_vec_row_major(data_count, iteration_count, &mut dummy_sum));
    println!("Multi-Vec Column-Major access: {} ms", multi_vec_column_major(data_count, iteration_count, &mut dummy_sum));

    //
    // Single Vec
    //
    println!("Vec Row-Major access: {} ms", vec_row_major(data_count, iteration_count, &mut dummy_sum));
    println!("Vec Column-Major access: {} ms", vec_column_major(data_count, iteration_count, &mut dummy_sum));

    //
    // Element-Wise Vec
    //
    println!("Vec Element-Wise access: {} ms", vec_elementwise(data_count, iteration_count, &mut dummy_sum));

    println!("");



    let data_count: usize = 64;
    println!("RUNNING ACCESS TESTS WITH 64x64x64 data elements for {} iterations!", iteration_count);
    println!("=============================================================");

    // Comment these back in to get a stack overflow - how fun!
    // //
    // // Multi-Array
    // //
    // println!("Multi-Array Row-Major access: {} ms", multi_array_64_row_major(iteration_count, &mut dummy_sum));
    // println!("Multi-Array Column-Major access: {} ms", multi_array_64_column_major(iteration_count, &mut dummy_sum));

    //
    // Multi-Vec
    //
    println!("Multi-Vec Row-Major access: {} ms", multi_vec_row_major(data_count, iteration_count, &mut dummy_sum));
    println!("Multi-Vec Column-Major access: {} ms", multi_vec_column_major(data_count, iteration_count, &mut dummy_sum));

    //
    // Single Vec
    //
    println!("Vec Row-Major access: {} ms", vec_row_major(data_count, iteration_count, &mut dummy_sum));
    println!("Vec Column-Major access: {} ms", vec_column_major(data_count, iteration_count, &mut dummy_sum));

    //
    // Element-Wise Vec
    //
    println!("Vec Element-Wise access: {} ms", vec_elementwise(data_count, iteration_count, &mut dummy_sum));

    println!("");



    let data_count: usize = 128;
    println!("RUNNING ACCESS TESTS WITH 128x128x128 data elements for {} iterations!", iteration_count);
    println!("=============================================================");

    // Comment these back in to get a stack overflow - how fun!
    // //
    // // Multi-Array
    // //
    // println!("Multi-Array Row-Major access: {} ms", multi_array_128_row_major(iteration_count, &mut dummy_sum));
    // println!("Multi-Array Column-Major access: {} ms", multi_array_128_column_major(iteration_count, &mut dummy_sum));

    //
    // Multi-Vec
    //
    println!("Multi-Vec Row-Major access: {} ms", multi_vec_row_major(data_count, iteration_count, &mut dummy_sum));
    println!("Multi-Vec Column-Major access: {} ms", multi_vec_column_major(data_count, iteration_count, &mut dummy_sum));

    //
    // Single Vec
    //
    println!("Vec Row-Major access: {} ms", vec_row_major(data_count, iteration_count, &mut dummy_sum));
    println!("Vec Column-Major access: {} ms", vec_column_major(data_count, iteration_count, &mut dummy_sum));

    //
    // Element-Wise Vec
    //
    println!("Vec Element-Wise access: {} ms", vec_elementwise(data_count, iteration_count, &mut dummy_sum));

    println!("");
}

fn multi_array_16_row_major(iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    const DATA_COUNT: usize = 16;
    let mut data: [[[i32; DATA_COUNT]; DATA_COUNT]; DATA_COUNT] = [[[0; DATA_COUNT]; DATA_COUNT]; DATA_COUNT];
    
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for x_index in 0..DATA_COUNT {
            for y_index in 0..DATA_COUNT {
                for z_index in 0..DATA_COUNT {
                    data[x_index][y_index][z_index] += 1;
                    *dummy_sum += data[x_index][y_index][z_index];
                }
            }
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn multi_array_16_column_major(iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    const DATA_COUNT: usize = 16;
    let mut data: [[[i32; DATA_COUNT]; DATA_COUNT]; DATA_COUNT] = [[[0; DATA_COUNT]; DATA_COUNT]; DATA_COUNT];
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for z_index in 0..DATA_COUNT {
            for y_index in 0..DATA_COUNT {
                for x_index in 0..DATA_COUNT {
                    data[x_index][y_index][z_index] += 1;
                    *dummy_sum += data[x_index][y_index][z_index];
                }
            }
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn multi_array_32_row_major(iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    const DATA_COUNT: usize = 32;
    let mut data: [[[i32; DATA_COUNT]; DATA_COUNT]; DATA_COUNT] = [[[0; DATA_COUNT]; DATA_COUNT]; DATA_COUNT];
    
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for x_index in 0..DATA_COUNT {
            for y_index in 0..DATA_COUNT {
                for z_index in 0..DATA_COUNT {
                    data[x_index][y_index][z_index] += 1;
                    *dummy_sum += data[x_index][y_index][z_index];
                }
            }
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn multi_array_32_column_major(iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    const DATA_COUNT: usize = 32;
    let mut data: [[[i32; DATA_COUNT]; DATA_COUNT]; DATA_COUNT] = [[[0; DATA_COUNT]; DATA_COUNT]; DATA_COUNT];
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for z_index in 0..DATA_COUNT {
            for y_index in 0..DATA_COUNT {
                for x_index in 0..DATA_COUNT {
                    data[x_index][y_index][z_index] += 1;
                    *dummy_sum += data[x_index][y_index][z_index];
                }
            }
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn multi_array_64_row_major(iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    const DATA_COUNT: usize = 64;
    let mut data: [[[i32; DATA_COUNT]; DATA_COUNT]; DATA_COUNT] = [[[0; DATA_COUNT]; DATA_COUNT]; DATA_COUNT];
    
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for x_index in 0..DATA_COUNT {
            for y_index in 0..DATA_COUNT {
                for z_index in 0..DATA_COUNT {
                    data[x_index][y_index][z_index] += 1;
                    *dummy_sum += data[x_index][y_index][z_index];
                }
            }
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn multi_array_64_column_major(iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    const DATA_COUNT: usize = 64;
    let mut data: [[[i32; DATA_COUNT]; DATA_COUNT]; DATA_COUNT] = [[[0; DATA_COUNT]; DATA_COUNT]; DATA_COUNT];
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for z_index in 0..DATA_COUNT {
            for y_index in 0..DATA_COUNT {
                for x_index in 0..DATA_COUNT {
                    data[x_index][y_index][z_index] += 1;
                    *dummy_sum += data[x_index][y_index][z_index];
                }
            }
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn multi_array_128_row_major(iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    const DATA_COUNT: usize = 128;
    let mut data: [[[i32; DATA_COUNT]; DATA_COUNT]; DATA_COUNT] = [[[0; DATA_COUNT]; DATA_COUNT]; DATA_COUNT];
    
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for x_index in 0..DATA_COUNT {
            for y_index in 0..DATA_COUNT {
                for z_index in 0..DATA_COUNT {
                    data[x_index][y_index][z_index] += 1;
                    *dummy_sum += data[x_index][y_index][z_index];
                }
            }
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn multi_array_128_column_major(iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    const DATA_COUNT: usize = 128;
    let mut data: [[[i32; DATA_COUNT]; DATA_COUNT]; DATA_COUNT] = [[[0; DATA_COUNT]; DATA_COUNT]; DATA_COUNT];
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for z_index in 0..DATA_COUNT {
            for y_index in 0..DATA_COUNT {
                for x_index in 0..DATA_COUNT {
                    data[x_index][y_index][z_index] += 1;
                    *dummy_sum += data[x_index][y_index][z_index];
                }
            }
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn multi_vec_row_major(data_count: usize, iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    let mut data: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; data_count]; data_count]; data_count];
    
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for x_index in 0..data_count {
            for y_index in 0..data_count {
                for z_index in 0..data_count {
                    data[x_index][y_index][z_index] += 1;
                    *dummy_sum += data[x_index][y_index][z_index];
                }
            }
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn multi_vec_column_major(data_count: usize, iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    let mut data: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; data_count]; data_count]; data_count];
    
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for z_index in 0..data_count {
            for y_index in 0..data_count {
                for x_index in 0..data_count {
                    data[x_index][y_index][z_index] += 1;
                    *dummy_sum += data[x_index][y_index][z_index];
                }
            }
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn vec_row_major(data_count: usize, iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    let mut data: Vec<i32> = vec![0; data_count * data_count * data_count];
    
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for x_index in 0..data_count {
            for y_index in 0..data_count {
                for z_index in 0..data_count {
                    data[x_index * data_count * data_count + y_index * data_count + z_index] += 1;
                    *dummy_sum += data[x_index * data_count * data_count + y_index * data_count + z_index];
                }
            }
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn vec_column_major(data_count: usize, iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    let mut data: Vec<i32> = vec![0; data_count * data_count * data_count];
    
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for z_index in 0..data_count {
            for y_index in 0..data_count {
                for x_index in 0..data_count {
                    data[x_index * data_count * data_count + y_index * data_count + z_index] += 1;
                    *dummy_sum += data[x_index * data_count * data_count + y_index * data_count + z_index];
                }
            }
        }   
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}

fn vec_elementwise(data_count: usize, iteration_count: usize, dummy_sum: &mut i32) -> f64 {
    let mut data: Vec<i32> = vec![0; data_count * data_count * data_count];
    
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for index in 0..data_count*data_count*data_count {
            data[index] += 1;
            *dummy_sum += data[index];
        }
    }
    let elapsed_time: Duration = now.elapsed();
    elapsed_time.as_millis() as f64
}