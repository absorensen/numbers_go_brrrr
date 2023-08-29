use std::{time::{Duration, Instant}, sync::{Mutex, Arc}};
use itertools::Itertools;
use rayon::prelude::{ParallelIterator, IntoParallelRefMutIterator};

#[inline(always)]
fn map_function(data: &mut [f32]){
    for index in 0..data.len() {
        let x: f32 = data[index];
        let mut x: f32 = x * x * x * x + x * x + x * x / x + x;

        for _ in 0..62 {
            x = x * 2.0 + 4.0 + 12.0 / 59.0;
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
fn fine_map_function(data: &mut f32) {
    let x: f32 = *data;
    let mut x: f32 = x * x * x * x + x * x + x * x / x + x;

    for _ in 0..62 {
        x = x * 2.0 + 4.0 + 12.0 / 59.0;
    }

    *data = x;
}

#[inline(always)]
fn fine_double_function(data: &mut f32) {
    *data *= 2.0;
}

fn parallelism(element_count: usize, thread_count: usize, chunk_size: usize, iteration_count: usize, single_thread: bool, rayon: bool, crossbeam_scope: bool, crossbeam_task_queue: bool, crossbeam_atomic_chunks: bool ) {
    let mut data: Vec<f32> = (0..element_count).into_iter().map(|x| x as f32).collect();
    let mut atomic_data_double: Vec<f32> = data.clone();
    let mut atomic_data_map: Vec<f32> = data.clone();
    let mut fine_data: Vec<f32> = (0..element_count).into_iter().map(|x| x as f32).collect();

    let mut data_chunks: Vec<&mut [f32]> = data.chunks_mut(chunk_size).collect_vec();



    //
    // Double Function
    //
    println!("DOUBLE FUNCTION:");
    if single_thread {
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            for data_chunk in &mut data_chunks {
                    double_function(*data_chunk);
            }
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for single threaded", elapsed_time.as_millis() as f64);
    }

    if rayon {
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let _result: () = data_chunks.par_iter_mut().map(|data_chunk| double_function(data_chunk) ).collect();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for coarse-grained rayon", elapsed_time.as_millis() as f64);


        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let _result: () = fine_data.par_iter_mut().map(|data| fine_double_function(data) ).collect();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for fine-grained rayon", elapsed_time.as_millis() as f64);
    }

    if crossbeam_scope {
        // This is to prevent this part from launching
        // thousands of threads and oversubscribing
        // instead of just launch some threads
        // and distributing the work
        if (element_count / chunk_size) < 1000 {
            let now: Instant = Instant::now();
            for _ in 0..iteration_count {
                crossbeam::scope(|spawner| {
                    for data_chunk in &mut data_chunks {
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
            let task_queue = Arc::new(Mutex::new(data_chunks.iter_mut()));
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
            let chunks = AtomicChunksMut::new(&mut atomic_data_double, chunk_size);
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
            for data_chunk in &mut data_chunks {
                    map_function(*data_chunk);
            }
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for single threaded", elapsed_time.as_millis() as f64);
    }

    if rayon {
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let _result: () = data_chunks.par_iter_mut().map(|data_chunk| map_function(data_chunk) ).collect();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for coarse-grained rayon", elapsed_time.as_millis() as f64);


        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let _result: () = fine_data.par_iter_mut().map(|data| fine_map_function(data) ).collect();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for fine-grained rayon", elapsed_time.as_millis() as f64);
    }

    if crossbeam_scope {
        // This is to prevent this part from launching
        // thousands of threads and oversubscribing
        // instead of just launch some threads
        // and distributing the work
        if (element_count / chunk_size) < 1000 {
            let now: Instant = Instant::now();
            for _ in 0..iteration_count {
                crossbeam::scope(|spawner| {
                    for data_chunk in &mut data_chunks {
                        spawner.spawn(move |_| {
                            map_function(data_chunk);
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
            let task_queue = Arc::new(Mutex::new(data_chunks.iter_mut()));
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
                                    map_function(data_chunk);
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
            let chunks = AtomicChunksMut::new(&mut atomic_data_map, chunk_size);
            let iteration_now: Instant = Instant::now();
            crossbeam::scope(|spawner| {
                for _ in 0..thread_count {
                    spawner.spawn(|_| {
                        for (_, chunk) in &chunks {
                            map_function(chunk);
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
    let element_count: usize = 10_000_000;
    let iteration_count: usize = 100;
    let thread_count: usize = 8;
    let chunk_size: usize = element_count / (thread_count * 32);

    let single_thread: bool = true;
    let rayon: bool = true;
    let crossbeam_scope: bool = true;
    let crossbeam_task_queue: bool = true;
    let crossbeam_atomic_chunks: bool = true;

    println!("Parallelism:");
    println!("================");
    println!("Element Count: {}", element_count);
    println!("Iteration Count: {}", iteration_count);
    println!("Thread Count: {}", thread_count);
    println!("Chunk Size: {} resulting in {} chunks", chunk_size, element_count / chunk_size + 1);
    println!("");
    parallelism(element_count, thread_count, chunk_size, iteration_count, single_thread, rayon, crossbeam_scope, crossbeam_task_queue, crossbeam_atomic_chunks );
    println!("");
    println!("");
}
