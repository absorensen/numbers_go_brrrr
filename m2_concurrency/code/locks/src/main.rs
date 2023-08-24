use std::{time::{Duration, Instant}, sync::{Mutex, Arc}};
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

fn crossbeam(element_count: usize, thread_count: usize, chunk_size: usize, iteration_count: usize) {
    let input: Vec<f32> = (0..element_count).into_iter().map(|x| x as f32).collect();
    let mut output: Vec<f32> = (0..element_count).into_iter().map(|_| 0.0 ).collect();
    let fine_input: Vec<f32> = (0..element_count).into_iter().map(|x| x as f32).collect();
    let mut fine_output: Vec<f32> = (0..element_count).into_iter().map(|_| 0.0 ).collect();

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

    {
        let mut total_time: u128 = 0;
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let task_queue = Arc::new(Mutex::new(zipped_chunks.iter_mut()));
            let iteration_now: Instant = Instant::now();
            crossbeam::scope(|spawner| {
                for _ in 0..thread_count {
                    let task_queue_handle = Arc::clone(&task_queue);
                    spawner.spawn(move |_| {
                        loop {
                            match {
                                let mut data: std::sync::MutexGuard<'_, std::slice::IterMut<'_, (&[f32], &mut [f32])>> = task_queue_handle.lock().unwrap();
                                data.next()
                            }
                            {
                                None => { return; }
                                Some((input_chunk, output_chunk)) => {
                                    double_function(input_chunk, output_chunk);
                                }
                            }
                        }
                    });
                }
            }).unwrap();
            total_time += iteration_now.elapsed().as_millis();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for crossbeam task queue double function", elapsed_time.as_millis() as f64);
        println!("{} ms for crossbeam task queue double function when discounting queue creation", total_time as f64);
        println!("");
    }


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

    {
        let mut total_time: u128 = 0;
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let task_queue = Arc::new(Mutex::new(zipped_chunks.iter_mut()));
            let iteration_now: Instant = Instant::now();
            crossbeam::scope(|spawner| {
                for _ in 0..thread_count {
                    let task_queue_handle = Arc::clone(&task_queue);
                    spawner.spawn(move |_| {
                        loop {
                            match {
                                let mut data: std::sync::MutexGuard<'_, std::slice::IterMut<'_, (&[f32], &mut [f32])>> = task_queue_handle.lock().unwrap();
                                data.next()
                            }
                            {
                                None => { return; }
                                Some((input_chunk, output_chunk)) => {
                                    map_function(input_chunk, output_chunk);
                                }
                            }
                        }
                    });
                }
            }).unwrap();
            total_time += iteration_now.elapsed().as_millis();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for crossbeam task queue map function", elapsed_time.as_millis() as f64);
        println!("{} ms for crossbeam task queue map function when discounting queue creation", total_time as f64);
        println!("");
    }
}

fn main() {
    let element_count: usize = 10_000_000;
    let iteration_count: usize = 10;
    let thread_count: usize = 8;
    let chunk_size: usize = element_count / (thread_count * 32);

    println!("Crossbeam Task Queue:");
    println!("================");
    crossbeam(element_count, thread_count, chunk_size, iteration_count);
    println!("");
    println!("");
}
