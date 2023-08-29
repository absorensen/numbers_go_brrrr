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

fn main() {
    let thread_count: u32 = 8;
    let repetition_count: u32 = 3;
    let wait_time: u64 = 40;

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
