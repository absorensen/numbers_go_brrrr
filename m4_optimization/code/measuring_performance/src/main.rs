use std::time::{Duration, Instant};


fn base_function(loop_count_outer: u32, loop_count_inner: u32) -> f32 {
    let mut sum: f32 = 0.0;
    for _ in 0..loop_count_outer {
        let mut x: f32 = 1.0;
        for _ in 0..loop_count_inner {
            x *= 2.0 + x;
        }
        sum += x;
    }
    sum
}

fn a() -> f32 {
    base_function(100, 100)
}

fn a_erratic() -> f32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    if rng.gen::<f32>() < (59.0/60.0) {
        base_function(100, 100)
    } else {
        base_function(10_000, 100)
    }
}

fn b() -> f32 {
    base_function(100, 100)
}

fn c() -> f32 {
    base_function(10_000, 100)
}

fn d() -> f32 {
    base_function(10, 100)
}

fn version_0() {
    let mut sum: f32 = 0.0;
    sum += a();
    sum += b();
    sum += c();
    sum += d();
    println!("sum was {}", sum);
}

fn version_1() {
    let mut sum: f32 = 0.0;
    let now: Instant = Instant::now();
    sum += a();
    sum += b();
    sum += c();
    sum += d();
    let elapsed_time: Duration = now.elapsed();

    println!("version_1 ran for {} milliseconds", elapsed_time.as_micros());
    println!("sum was {}", sum);
}

fn version_2() {
    let mut sum: f32 = 0.0;

    let now_a: Instant = Instant::now();
    sum += a();
    let elapsed_time_a: Duration = now_a.elapsed();
    println!("version_2_a ran for {} milliseconds", elapsed_time_a.as_millis());

    let now_b: Instant = Instant::now();
    sum += b();
    let elapsed_time_b: Duration = now_b.elapsed();
    println!("version_2_b ran for {} milliseconds", elapsed_time_b.as_millis());

    let now_c: Instant = Instant::now();
    sum += c();
    let elapsed_time_c: Duration = now_c.elapsed();
    println!("version_2_c ran for {} milliseconds", elapsed_time_c.as_millis());

    let now_d: Instant = Instant::now();
    sum += d();
    let elapsed_time_d: Duration = now_d.elapsed();
    println!("version_2_d ran for {} milliseconds", elapsed_time_d.as_millis());

    println!("sum was {}", sum);
}


fn version_3() {
    let mut sum: f32 = 0.0;
    let iteration_count: u32 = 2_000;

    let now_a: Instant = Instant::now();
    for _ in 0..iteration_count {
        sum += a();
    }
    let elapsed_time_a: Duration = now_a.elapsed();
    println!("version_3_a ran for {} milliseconds", elapsed_time_a.as_millis());

    let now_b: Instant = Instant::now();
    for _ in 0..iteration_count {
        sum += b();
    }
    let elapsed_time_b: Duration = now_b.elapsed();
    println!("version_3_b ran for {} milliseconds", elapsed_time_b.as_millis());

    let now_c: Instant = Instant::now();
    for _ in 0..iteration_count {
        sum += c();
    }
    let elapsed_time_c: Duration = now_c.elapsed();
    println!("version_3_c ran for {} milliseconds", elapsed_time_c.as_millis());

    let now_d: Instant = Instant::now();
    for _ in 0..iteration_count {
        sum += d();
    }
    let elapsed_time_d: Duration = now_d.elapsed();
    println!("version_3_d ran for {} milliseconds", elapsed_time_d.as_millis());

    println!("sum was {}", sum);
}

fn version_4() {
    let mut sum: f32 = 0.0;
    let millisecond_limit: u128 = 2_000;

    let mut execution_count_a: u128 = 0;
    let now_a: Instant = Instant::now();
    while now_a.elapsed().as_millis() < millisecond_limit {
        sum += a();
        execution_count_a += 1;
    }
    println!("version_4_a executed {} times in {} milliseconds", execution_count_a, millisecond_limit);

    let mut execution_count_b: u128 = 0;
    let now_b: Instant = Instant::now();
    while now_b.elapsed().as_millis() < millisecond_limit {
        sum += b();
        execution_count_b += 1;
    }
    println!("version_4_b executed {} times in {} milliseconds", execution_count_b, millisecond_limit);
    
    let mut execution_count_c: u128 = 0;
    let now_c: Instant = Instant::now();
    while now_c.elapsed().as_millis() < millisecond_limit {
        sum += c();
        execution_count_c += 1;
    }
    println!("version_4_c executed {} times in {} milliseconds", execution_count_c, millisecond_limit);
    
    let mut execution_count_d: u128 = 0;
    let now_d: Instant = Instant::now();
    while now_d.elapsed().as_millis() < millisecond_limit {
        sum += d();
        execution_count_d += 1;
    }
    println!("version_4_d executed {} times in {} milliseconds", execution_count_d, millisecond_limit);

    println!("sum was {}", sum);
}

fn get_mean_and_variance(measurements: &Vec<u128>) -> (f32, f32) {
    let sum: u128 = measurements.iter().sum();
    let mean: f32 = sum as f32 / measurements.len() as f32;

    let mut variance: f32 = 0.0;
    for element in measurements {
        let squared: f32 = (*element as f32 - mean) * (*element as f32 - mean);
        variance += squared;
    }

    variance /= measurements.len() as f32;

    (mean, variance)
}

fn version_5() {
    let mut sum: f32 = 0.0;
    let measurement_count: usize = 2_000;
    let mut measurements: Vec<u128> = vec![0; measurement_count];

    for measurement_index in 0..measurement_count {
        let now: Instant = Instant::now();
        sum += a();
        let elapsed_time: Duration = now.elapsed();
        measurements[measurement_index] = elapsed_time.as_micros();
    }
    let (mean, variance) : (f32, f32) = get_mean_and_variance(&measurements);
    println!("version_5_a ran with mean {} microseconds and variance {} microseconds squared", mean, variance);

    for measurement_index in 0..measurement_count {
        let now: Instant = Instant::now();
        sum += b();
        let elapsed_time: Duration = now.elapsed();
        measurements[measurement_index] = elapsed_time.as_micros();
    }
    let (mean, variance) : (f32, f32) = get_mean_and_variance(&measurements);
    println!("version_5_b ran with mean {} microseconds and variance {} microseconds squared", mean, variance);
    
    for measurement_index in 0..measurement_count {
        let now: Instant = Instant::now();
        sum += c();
        let elapsed_time: Duration = now.elapsed();
        measurements[measurement_index] = elapsed_time.as_micros();
    }
    let (mean, variance) : (f32, f32) = get_mean_and_variance(&measurements);
    println!("version_5_c ran with mean {} microseconds and variance {} microseconds squared", mean, variance);

    for measurement_index in 0..measurement_count {
        let now: Instant = Instant::now();
        sum += d();
        let elapsed_time: Duration = now.elapsed();
        measurements[measurement_index] = elapsed_time.as_micros();
    }
    let (mean, variance) : (f32, f32) = get_mean_and_variance(&measurements);
    println!("version_5_d ran with mean {} microseconds and variance {} microseconds squared", mean, variance);

    println!("sum was {}", sum);
}

fn version_6() {
    let mut sum: f32 = 0.0;
    let measurement_count: usize = 2_000;
    let mut measurements: Vec<u128> = vec![0; measurement_count];

    for measurement_index in 0..measurement_count {
        let now: Instant = Instant::now();
        sum += a_erratic();
        let elapsed_time: Duration = now.elapsed();
        measurements[measurement_index] = elapsed_time.as_micros();
    }
    let (mean, variance) : (f32, f32) = get_mean_and_variance(&measurements);
    println!("version_6_a ran with mean {} microseconds and variance {} microseconds squared", mean, variance);

    for measurement_index in 0..measurement_count {
        let now: Instant = Instant::now();
        sum += b();
        let elapsed_time: Duration = now.elapsed();
        measurements[measurement_index] = elapsed_time.as_micros();
    }
    let (mean, variance) : (f32, f32) = get_mean_and_variance(&measurements);
    println!("version_6_b ran with mean {} microseconds and variance {} microseconds squared", mean, variance);
    
    for measurement_index in 0..measurement_count {
        let now: Instant = Instant::now();
        sum += c();
        let elapsed_time: Duration = now.elapsed();
        measurements[measurement_index] = elapsed_time.as_micros();
    }
    let (mean, variance) : (f32, f32) = get_mean_and_variance(&measurements);
    println!("version_6_c ran with mean {} microseconds and variance {} microseconds squared", mean, variance);

    for measurement_index in 0..measurement_count {
        let now: Instant = Instant::now();
        sum += d();
        let elapsed_time: Duration = now.elapsed();
        measurements[measurement_index] = elapsed_time.as_micros();
    }
    let (mean, variance) : (f32, f32) = get_mean_and_variance(&measurements);
    println!("version_6_d ran with mean {} microseconds and variance {} microseconds squared", mean, variance);

    println!("sum was {}", sum);
}

fn main() {
    version_6();
}
