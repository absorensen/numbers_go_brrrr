use std::{thread::{self, JoinHandle}, time::{Duration, Instant}};
use itertools::Itertools;
use rayon::prelude::{ParallelIterator, IntoParallelRefMutIterator, IndexedParallelIterator, IntoParallelRefIterator};

#[inline(always)]
fn map_function(input: &[f32], output: &mut [f32]) {
    for index in 0..input.len() {
        let x: f32 = input[index];
        let mut x: f32 = x * x * x * x + x * x + x * x / x + x;

        for _ in 0..62 {
            x = x * 2.0 + 4.0 + 12.0 / 59.0;
        }

        output[index] = x;
    }
}

#[inline(always)]
fn double_function(input: &[f32], output: &mut [f32]) {
    for index in 0..input.len() {
        output[index] = 2.0 * input[index];
    }
}

#[inline(always)]
fn fine_map_function(input: &f32, output: &mut f32) {
    let x: f32 = *input;
    let mut x: f32 = x * x * x * x + x * x + x * x / x + x;

    for _ in 0..62 {
        x = x * 2.0 + 4.0 + 12.0 / 59.0;
    }

    *output = x;
}

#[inline(always)]
fn fine_double_function(input: &f32, output: &mut f32) {
    *output = 2.0 * *input;
}

fn print_thread(name: String, repetition_count: u32, wait_time: u64) {
    for repetition in 0..repetition_count {
        println!("Thread {} Print {}", name, repetition);
        thread::sleep(Duration::from_millis(wait_time));
    }
}

fn basic_threading(thread_count: u32, repetition_count: u32, wait_time: u64) {
    for thread_index in 0..thread_count {
        // We get access from use std::thread
        // The closure arguments ( || ) are for
        // moving any variables to be owned by this
        // thread.
        // Note the function after the ||.
        // This can get out of hand fairly quickly
        // so I'm just gonna call a function from
        // here.
        thread::spawn(move || {
            print_thread(thread_index.to_string(), repetition_count, wait_time);
        });
        // All the threads we just launched, will exist until
        // the main thread is terminated. This will happen
        // when the entire program ends. We haven't determined
        // a way to stop the threads yet.
    }

    print_thread("MAIN".to_string(), repetition_count, wait_time);
}

fn basic_threading_with_termination(thread_count: u32, repetition_count: u32, wait_time: u64) {
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for thread_index in 0..thread_count {
        handles.push(thread::spawn(move || {
            print_thread(thread_index.to_string(), repetition_count, wait_time);
        }));
    }

    print_thread("MAIN".to_string(), repetition_count, wait_time);

    // Wait until each thread has completed its tasks
    for handle in handles {
        handle.join().unwrap();
    }
}

fn basic_threading_with_scope(thread_count: u32, repetition_count: u32, wait_time: u64) {
    crossbeam::scope(|spawner| {
        for thread_index in 0..thread_count {
            spawner.spawn(move |_| {
                print_thread(thread_index.to_string(), repetition_count, wait_time);
            });
        }
    }).unwrap();
    
    // Note that the MAIN print does not happen until
    // the very end, because now we effectively have joins
    // on every thread at the end of crossbeam::scope's
    // scope.
    print_thread("MAIN".to_string(), repetition_count, wait_time);
}

fn crossbeam(element_count: usize, iteration_count: usize, thread_count: u32) {
    let input: Vec<f32> = (0..element_count).into_iter().map(|x| x as f32).collect();
    let mut output: Vec<f32> = (0..element_count).into_iter().map(|_| 0.0 ).collect();
    let fine_input: Vec<f32> = (0..element_count).into_iter().map(|x| x as f32).collect();
    let mut fine_output: Vec<f32> = (0..element_count).into_iter().map(|_| 0.0 ).collect();
    let chunk_size: usize = element_count / thread_count as usize;

    let input_chunks: Vec<&[f32]> = input.chunks(chunk_size).collect_vec();
    let output_chunks: Vec<&mut [f32]> = output.chunks_mut(chunk_size).collect_vec();

    let mut zipped_chunks: Vec<(&[f32], &mut [f32])> = input_chunks.into_iter().zip(output_chunks).collect_vec();


    //
    // Double Function
    //

    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for (input_chunk, output_chunk) in &mut zipped_chunks {
                double_function(input_chunk, output_chunk);
        }
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for single threaded double function", elapsed_time.as_millis() as f64);


    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        let result: () = zipped_chunks.par_iter_mut().map(|(input_chunk, output_chunk)| double_function(input_chunk, output_chunk) ).collect();
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for rayon double function", elapsed_time.as_millis() as f64);


    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        let result: () = fine_input.par_iter().zip(&mut fine_output).map(|(input, output)| fine_double_function(input, output) ).collect();
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for fine-grained rayon double function", elapsed_time.as_millis() as f64);


    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        crossbeam::scope(|spawner| {
            for (input_chunk, output_chunk) in &mut zipped_chunks {
                spawner.spawn(move |_| {
                    double_function(input_chunk, output_chunk);
                });
            }
        }).unwrap();
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for crossbeam double function", elapsed_time.as_millis() as f64);



    //
    // Map Function
    //

    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for (input_chunk, output_chunk) in &mut zipped_chunks {
                map_function(input_chunk, output_chunk);
        }
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for single threaded map function", elapsed_time.as_millis() as f64);

    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        let result: () = zipped_chunks.par_iter_mut().map(|(input_chunk, output_chunk)| map_function(input_chunk, output_chunk) ).collect();
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for rayon map function", elapsed_time.as_millis() as f64);

    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        let result: () = fine_input.par_iter().zip(&mut fine_output).map(|(input, output)| fine_map_function(input, output) ).collect();
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for fine-grained rayon double function", elapsed_time.as_millis() as f64);

    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        crossbeam::scope(|spawner| {
            for (input_chunk, output_chunk) in &mut zipped_chunks {
                spawner.spawn(move |_| {
                    map_function(input_chunk, output_chunk);
                });
            }
        }).unwrap();
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for crossbeam map function", elapsed_time.as_millis() as f64);
}

fn main() {
    let benchmark_level_3: bool = true;
    if benchmark_level_3 {
        let element_count: usize = 10_000_000;
        let iteration_count: usize = 10;
        let thread_count: u32 = 8;

        println!("RUNNING LEVEL 3");
        println!("Crossbeam Chunking and Scope:");
        println!("================");
        crossbeam(element_count, iteration_count, thread_count);
        println!("");
        println!("");
    } else {
        let thread_count: u32 = 16;
        let repetition_count: u32 = 3;
        let wait_time: u64 = 40;

        println!("RUNNING LEVEL 2");
        println!("Basic Threading:");
        println!("================");
        basic_threading(thread_count, repetition_count, wait_time);
        println!("");
        println!("");

        println!("Basic Threading with Termination:");
        println!("=================================");
        basic_threading_with_termination(thread_count, repetition_count, wait_time);
        println!("");
        println!("");

        println!("Basic Threading with Scope:");
        println!("===========================");
        basic_threading_with_scope(thread_count, repetition_count, wait_time);
        println!("");
        println!("");
    }
}
