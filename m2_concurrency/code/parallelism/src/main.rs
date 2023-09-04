use std::{time::{Duration, Instant}, sync::{Mutex, Arc}};
use itertools::Itertools;
use rand::rngs::ThreadRng;
use rand::prelude::*;
use rayon::prelude::{ParallelIterator, IntoParallelRefMutIterator};

#[inline(always)]
fn map_function(complexity: usize, escape_probability: f32, data: &mut [f32]){
    let mut rng: ThreadRng = thread_rng();
    for index in 0..data.len() {
        let x: f32 = data[index];
        let mut x: f32 = x * x * x * x + x * x + x * x / x + x;

        if 0.0 < escape_probability {
            for _ in 0..complexity {
                let escape: f32 = rng.gen();
                if escape < escape_probability { break; }
                x = x * 2.0 + 4.0 + 12.0 / 59.0;
            }
        } else {
            for _ in 0..complexity {
                x = x * 2.0 + 4.0 + 12.0 / 59.0;
            }
        }


        data[index] = x;
    }
}

#[inline(always)]
fn double_function(data: &mut [f32]) {
    for index in 0..data.len() {
        data[index] *= 2.0;
    }
}

#[inline(always)]
fn fine_map_function(complexity: usize, escape_probability: f32, data: &mut f32) {
    let mut rng: ThreadRng = thread_rng();
    let x: f32 = *data;
    let mut x: f32 = x * x * x * x + x * x + x * x / x + x;

    if escape_probability == 0.0 {
        for _ in 0..complexity {
            let escape: f32 = rng.gen();
            if escape < escape_probability { break; }
            x = x * 2.0 + 4.0 + 12.0 / 59.0;
        }
    } else {
        for _ in 0..complexity {
            x = x * 2.0 + 4.0 + 12.0 / 59.0;
        }
    }

    *data = x;
}

#[inline(always)]
fn fine_double_function(data: &mut f32) {
    *data *= 2.0;
}

fn parallelism(
    double_element_count: usize,
    map_element_count: usize, 
    thread_count: usize, 
    chunk_size: usize, 
    iteration_count: usize, 
    complexity: usize, 
    escape_probability: f32, 
    single_thread: bool, 
    rayon: bool, 
    crossbeam_scope: bool, 
    crossbeam_task_queue: bool, 
    crossbeam_atomic_chunks: bool ) {

    let mut double_data: Vec<f32> = (0..double_element_count).into_iter().map(|x| x as f32).collect();
    let mut double_atomic_data: Vec<f32> = double_data.clone();
    let mut double_fine_data: Vec<f32> = (0..double_element_count).into_iter().map(|x| x as f32).collect();

    let mut double_data_chunks: Vec<&mut [f32]> = double_data.chunks_mut(chunk_size).collect_vec();


    let mut map_data: Vec<f32> = (0..map_element_count).into_iter().map(|x| x as f32).collect();
    let mut map_atomic_data: Vec<f32> = map_data.clone();
    let mut map_fine_data: Vec<f32> = (0..map_element_count).into_iter().map(|x| x as f32).collect();

    let mut map_data_chunks: Vec<&mut [f32]> = map_data.chunks_mut(chunk_size).collect_vec();



    //
    // Double Function
    //
    println!("DOUBLE FUNCTION:");
    if single_thread {
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            for data_chunk in &mut double_data_chunks {
                    double_function(*data_chunk);
            }
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for single threaded", elapsed_time.as_millis() as f64);
    }

    if rayon {
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let _result: () = double_data_chunks.par_iter_mut().map(|data_chunk| double_function(data_chunk) ).collect();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for coarse-grained rayon", elapsed_time.as_millis() as f64);


        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let _result: () = double_fine_data.par_iter_mut().map(|data| fine_double_function(data) ).collect();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for fine-grained rayon", elapsed_time.as_millis() as f64);
    }

    if crossbeam_scope {
        // This is to prevent this part from launching
        // thousands of threads and oversubscribing
        // instead of just launch some threads
        // and distributing the work
        if (double_element_count / chunk_size) < 1000 {
            let now: Instant = Instant::now();
            for _ in 0..iteration_count {
                crossbeam::scope(|spawner| {
                    for data_chunk in &mut double_data_chunks {
                        spawner.spawn(move |_| {
                            double_function(data_chunk);
                        });
                    }
                }).unwrap();
            }
            let elapsed_time: Duration = now.elapsed();
            println!("{} ms for crossbeam scope", elapsed_time.as_millis() as f64);
        } else {
            println!("Omitted crossbeam scope due to too many threads to launch.");
        }
    }

    if crossbeam_task_queue {
        let mut total_time: u128 = 0;
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let task_queue = Arc::new(Mutex::new(double_data_chunks.iter_mut()));
            let iteration_now: Instant = Instant::now();
            crossbeam::scope(|spawner| {
                for _ in 0..thread_count {
                    let task_queue_handle = Arc::clone(&task_queue);
                    spawner.spawn(move |_| {
                        loop {
                            match {
                                let mut data = task_queue_handle.lock().unwrap();
                                data.next()
                            }
                            {
                                None => { return; }
                                Some(data_chunk) => {
                                    double_function(data_chunk);
                                }
                            }
                        }
                    });
                }
            }).unwrap();
            total_time += iteration_now.elapsed().as_millis();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for crossbeam task queue", elapsed_time.as_millis() as f64);
        println!("{} ms for crossbeam task queue when discounting queue creation", total_time as f64);
    }

    if crossbeam_atomic_chunks {
        use atomic_chunks_mut::AtomicChunksMut;
        let mut total_time: u128 = 0;
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
        {
            let chunks = AtomicChunksMut::new(&mut double_atomic_data, chunk_size);
            let iteration_now: Instant = Instant::now();
            crossbeam::scope(|spawner| {
                for _ in 0..thread_count {
                    spawner.spawn(|_| {
                        for (_, chunk) in &chunks {
                            double_function(chunk);
                        }
                    });
                }
            }).unwrap();
            total_time += iteration_now.elapsed().as_millis();

        }
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for crossbeam atomic chunks", elapsed_time.as_millis() as f64);
        println!("{} ms for crossbeam atomic chunks when discounting iterator creation", total_time as f64);
    }

    println!("");


    //
    // Map Function
    //
    if single_thread {
        println!("MAP FUNCTION:");
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            for data_chunk in &mut map_data_chunks {
                    map_function(complexity, escape_probability, *data_chunk);
            }
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for single threaded", elapsed_time.as_millis() as f64);
    }

    if rayon {
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let _result: () = map_data_chunks.par_iter_mut().map(|data_chunk| map_function(complexity, escape_probability, data_chunk) ).collect();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for coarse-grained rayon", elapsed_time.as_millis() as f64);


        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let _result: () = map_fine_data.par_iter_mut().map(|data| fine_map_function(complexity, escape_probability, data) ).collect();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for fine-grained rayon", elapsed_time.as_millis() as f64);
    }

    if crossbeam_scope {
        // This is to prevent this part from launching
        // thousands of threads and oversubscribing
        // instead of just launch some threads
        // and distributing the work
        if (map_element_count / chunk_size) < 1000 {
            let now: Instant = Instant::now();
            for _ in 0..iteration_count {
                crossbeam::scope(|spawner| {
                    for data_chunk in &mut map_data_chunks {
                        spawner.spawn(move |_| {
                            map_function(complexity, escape_probability, data_chunk);
                        });
                    }
                }).unwrap();
            }
            let elapsed_time: Duration = now.elapsed();
            println!("{} ms for crossbeam scope", elapsed_time.as_millis() as f64);
        } else {
            println!("Omitted crossbeam scope due to too many threads to launch.");
        }
    }


    if crossbeam_task_queue {
        let mut total_time: u128 = 0;
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let task_queue = Arc::new(Mutex::new(map_data_chunks.iter_mut()));
            let iteration_now: Instant = Instant::now();
            crossbeam::scope(|spawner| {
                for _ in 0..thread_count {
                    let task_queue_handle = Arc::clone(&task_queue);
                    spawner.spawn(move |_| {
                        loop {
                            match {
                                let mut data = task_queue_handle.lock().unwrap();
                                data.next()
                            }
                            {
                                None => { return; }
                                Some(data_chunk) => {
                                    map_function(complexity, escape_probability, data_chunk);
                                }
                            }
                        }
                    });
                }
            }).unwrap();
            total_time += iteration_now.elapsed().as_millis();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for crossbeam task queue", elapsed_time.as_millis() as f64);
        println!("{} ms for crossbeam task queue when discounting queue creation", total_time as f64);
    }

    if crossbeam_atomic_chunks {
        use atomic_chunks_mut::AtomicChunksMut;
        let mut total_time: u128 = 0;
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
        {
            let chunks = AtomicChunksMut::new(&mut map_atomic_data, chunk_size);
            let iteration_now: Instant = Instant::now();
            crossbeam::scope(|spawner| {
                for _ in 0..thread_count {
                    spawner.spawn(|_| {
                        for (_, chunk) in &chunks {
                            map_function(complexity, escape_probability, chunk);
                        }
                    });
                }
            }).unwrap();
            total_time += iteration_now.elapsed().as_millis();
        }

        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for crossbeam atomic chunks", elapsed_time.as_millis() as f64);
        println!("{} ms for crossbeam atomic chunks when discounting iterator creation", total_time as f64);
    }

    println!("");

}

fn main() {
    // How many elements will be processed by the simple function
    let double_element_count: usize = 100_000_000;
    // How many elemenets will be processed by the complex function
    let map_element_count: usize = 1_000_000;
    // How many times we will be going through the entire data set
    let iteration_count: usize = 100; 
    // How many threads we will launch in the cases where we aren't
    // launching a thread per chunk and are explicitly launching threads
    let thread_count: usize = 8; 
    // The size of the chunks we will partition the data in
    let chunk_size: usize = map_element_count / (thread_count * 32);
    // The probability of escaping the loop in the map function
    let escape_probability: f32 = 0.0;
    // The maximum amount of loops in the map function
    let complexity: usize = 62;

    let single_thread: bool = true;
    let rayon: bool = true;
    let crossbeam_scope: bool = true;
    let crossbeam_task_queue: bool = true;
    let crossbeam_atomic_chunks: bool = true;

    println!("Parallelism:");
    println!("================");
    println!("Double Element Count: {}", double_element_count);
    println!("Map Element Count: {}", map_element_count);
    println!("Iteration Count: {}", iteration_count);
    println!("Thread Count: {}", thread_count);
    println!("Chunk Size: {} resulting in {} chunks", chunk_size, map_element_count / chunk_size + 1);
    println!("Escape Probability: {}", escape_probability);
    println!("Complexity: {}", complexity);
    println!("");
    parallelism(
        double_element_count,
        map_element_count, 
        thread_count, 
        chunk_size, 
        iteration_count, 
        complexity, 
        escape_probability, 
        single_thread, 
        rayon, 
        crossbeam_scope, 
        crossbeam_task_queue, 
        crossbeam_atomic_chunks 
    );
    println!("");
    println!("");
}
