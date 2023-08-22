use std::time::{Instant, Duration};
use rand::prelude::*;
use rayon::prelude::*;


#[inline(always)]
fn map_function(x: f32) -> f32 {
    let mut x: f32 = x * x * x * x + x * x + x * x / x + x;

    for _ in 0..62 {
        x = x * 2.0 + 4.0 + 12.0 / 59.0;
    }

    x
}

fn level_2() {
    let element_count: usize = 10_000_000;
    let iteration_count: usize = 100;

    let mut data: Vec<f32> = (0..element_count).into_iter().map(|x| x as f32).collect();
    
    //
    // Map every element with simple multiplication
    //
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for element in &mut data {
            *element = *element * 3.14;
        }
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for for-loop double map", elapsed_time.as_millis() as f64);

    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        data = data.iter().map(|x| *x * 3.14).collect();
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for iterator double map", elapsed_time.as_millis() as f64);


    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        data = data.par_iter().map(|x| *x * 3.14).collect();
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for par iterator double map", elapsed_time.as_millis() as f64);



    //
    // Let's switch to a more complex map function!
    //
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        for element in &mut data {
            *element = map_function(*element);
        }
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for for-loop map_function", elapsed_time.as_millis() as f64);


    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        data = data.iter().map(|x| map_function(*x)).collect();
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for iterator map_function", elapsed_time.as_millis() as f64);


    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        data = data.par_iter().map(|x| map_function(*x)).collect();
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for par iterator map_function", elapsed_time.as_millis() as f64);

}

fn level_3() {
    //
    // Filter and count
    //
    let element_count: usize = 10_000_000;
    let iteration_count: usize = 100;

    println!("Running filter and count benchmark for {} elements with {} iterations!", element_count, iteration_count);

    let mut rng: ThreadRng = rand::thread_rng();
    let data: Vec<f32> = (0..element_count).map(|_| rng.gen_range(0.0..1.0)).collect();

    let now: Instant = Instant::now();
    let mut sums: usize = 0;
    for _ in 0..iteration_count {
        sums += data.iter().filter(|x| if 0.5 < **x { true } else { false }).count();
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{}% of elements were greater than 0.5", sums as f64 / iteration_count as f64 / element_count as f64);
    println!("{} ms for filter and sum", elapsed_time.as_millis() as f64);
    println!("");

    let now: Instant = Instant::now();
    let mut sums: usize = 0;
    for _ in 0..iteration_count {
        sums += data.par_iter().filter(|x| if 0.5 < **x { true } else { false }).count();
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{}% of elements were greater than 0.5", sums as f64 / iteration_count as f64 / element_count as f64);
    println!("{} ms for parallel filter and sum", elapsed_time.as_millis() as f64);
    println!("");
    println!("");

    //
    // Convolution
    //
    let element_count: usize = 1920*1080;
    let iteration_count: usize = 1000;
    let filter_sizes: Vec<usize> = vec![3, 5, 7, 9, 11, 13, 15];

    println!("Running convolution benchmark for {} elements with {} iterations!", element_count, iteration_count);
    println!("Filter sizes are: {:?}", filter_sizes);
    let mut rng: ThreadRng = rand::thread_rng();
    let data: Vec<f32> = (0..element_count).map(|_| rng.gen_range(0.0..1.0)).collect();
    let mut filters: Vec<Vec<f32>> = Vec::<Vec<f32>>::new();
    for size in &filter_sizes {
        let filter: Vec<f32> = (0..*size).map(|_| rng.gen_range(-1.0..1.0)).collect();
        filters.push(filter);
    }
    // Remove mutability to be sure.
    let filters: Vec<Vec<f32>> = filters;

    for (size, filter) in filter_sizes.iter().zip(&filters) {
        println!("Running filter size {}", *size);

        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let filtered: Vec<f32> = data.windows(*size).map(|x| {
                x.iter().zip(filter).map(|(element, filter)| *element * *filter).sum()
            } ).collect();
            filtered.iter().sum::<f32>();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for convolution", elapsed_time.as_millis() as f64);

        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            let filtered: Vec<f32> = data.par_windows(*size).map(|x| {
                x.iter().zip(filter).map(|(element, filter)| *element * *filter).sum()
            } ).collect();
            filtered.iter().sum::<f32>();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for parallel convolution", elapsed_time.as_millis() as f64);

        println!("");
    }

}

fn main() {
    let show_level_3: bool = false;

    if show_level_3 {
        println!("RUNNING LEVEL 3 BENCHMARKS!");
        println!("===========================");
        level_3();
    } else {
        println!("RUNNING LEVEL 2 BENCHMARKS!");
        println!("===========================");
        level_2();
    }
}
