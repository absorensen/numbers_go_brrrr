# Timing and Printing

As mentioned in the previous section there's a plethora of options in how to measure performance.
But in this section, let's look at what to measure.

Let's assume we have a basic program like the following -

=== "Rust"

    ```rust
    fn version_0() {
        a();
        b();
        c();
        d();
    }
    ```

I have written the full program, you can check it out in [m5_optimization::code::measuring_performance][0].
We have verified the correctness of our program. We have set up tests to continually ensure the continued
correctness of our program. We are now ready to improve the runtime of our program. I haven't actually done
this as I don't actually care about whether this specific program is correct. This is not the norm of course.
The fastest thing right now would be to either put in timing and print statements or to open up
our system wide performance monitor and see if anything glaringly wrong is happening. Let's go down
the time and print path.

In this case we will include some timing functionality from the standard library at the top of the file and
time the whole program.

=== "Rust"

    ```rust
    use std::time::{Duration, Instant};
    ```

Except, hold up. While doing this, I forgot a critical property that almost invariably happens in education
about performance. Because we are running very simplified programs, dead code elimination by the compiler has
a huge effect. I didn't properly use any of the results of the functions, so the Rust compiler, the helpful
helper that it is, decided to make my program much faster, by not doing any of the work as the work was used
nowhere. This was apparent when I timed all 4 functions, as the result was always either 100 or 200
nanoseconds. This sort of reeks of noise floor. So our new ```version_0()``` will look like this -

=== "Rust"

    ```rust
    fn version_0() {
        let mut sum: f32 = 0.0;
        sum += a();
        sum += b();
        sum += c();
        sum += d();
        println!("sum was {}", sum);
    }
    ```

This forces the Rust compiler to not eliminate our function calls. So now, we can time the entire program as
such -

=== "Rust"

    ```rust
    fn version_1() {
        let mut sum: f32 = 0.0;
        let now: Instant = Instant::now();
        sum += a();
        sum += b();
        sum += c();
        sum += d();
        let elapsed_time: Duration = now.elapsed();

        println!("version_1 ran for {} milliseconds", elapsed_time.as_millis());
        println!("sum was {}", sum);
    }
    ```

Now we get an output of 1 miliseconds. Clearly, we have chosen a time resolution which was too course. If we switch
to microseconds we get 1794 microseconds. But we don't know which function is taking the most time. I am going to keep
it in miliseconds in the outer function, but the time resolution will probably get finer the further down we go.

So let's time each function individually.

=== "Rust"

    ```rust
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
    ```

Which gives us this output -

=== "Rust"

    ```rust
    version_2_a ran for 0 milliseconds
    version_2_b ran for 0 milliseconds
    version_2_c ran for 1 milliseconds
    version_2_d ran for 0 milliseconds
    ```

Ok, so ```c()``` is likely to be taking up more of the execution time than the others. But we don't know how much.
Every other function could be 999 microseconds and function c could be 1000 microseconds. We could again increase the
resolution of the timing, but in this case we have something else we should think about. We have a background usage of
the system. We should try and turn off as many other processes as possible. Other processes might jump in and take
some time to do something which can disturb your measurements. Your system itself might also have a ramping up period.
As the load increases it might boost the clock frequency of the CPU to accomodate your program's needs.
A general rule of thumb is that your program should run for at least two seconds. We can either do this per function
or for the whole program. Let's just do this per function. We can then either keep track of how many executions of
the function were attained during a two second window or just do N loops of each function. Let's try both.

=== "Rust"

    ```rust
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
    ```

Getting the output -

=== "Rust"

    ```rust
    version_3_a ran for 22 milliseconds
    version_3_b ran for 23 milliseconds
    version_3_c ran for 2101 milliseconds
    version_3_d ran for 2 milliseconds
    ```

As you can see, measuring things this way, yields a quite unbalanced view of each function. So let's try number of
executions per second -

=== "Rust"

    ```rust
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
    ```

Now we get the executions per second -

=== "Rust"

    ```rust
    version_4_a executed 188668 times in 2000 milliseconds
    version_4_b executed 187431 times in 2000 milliseconds
    version_4_c executed 1888 times in 2000 milliseconds
    version_4_d executed 1804447 times in 2000 milliseconds
    ```

Clearly, it's no secret that function c takes a lot more time than the other functions. One of the real-life
applications we can compare this to is a real-time system where we care about the frames per second. We can either
measure the time it takes to process or render a single frame or we can measure how many frames per second we can
sustain. One crucial bit in an interactive real-time system is the variance. If you are churning out 60 frames
per second, but frame 0 takes 0.9999 seconds and the other frames each take 0.0000001 seconds, the user is getting a
very choppy experience. Even if it results in lower frames per second, like going from 60 to 50, distributing the
processing time a bit would be a smoother experience. You could also measure the average frames per second in a moving
window fashion while moving through a scene to see which areas of a scene might need tweaking performance wise.

What we'll look at instead is getting the variance calculated alongside our other metrics. This requires a bit
more admin work and we have to keep track of individual measurements in order to get a resonably accurate measurement.
In order to not disturb the characteristics of the function we are measuring too much we should probably pre-allocate
space for our measurements, which becomes our sample size. Continually pushing new values into a dynamic array
would result in lots of allocations, which would be bad. We are also now going to measure the mean execution time of
each function as well as the variance.

=== "Rust"

    ```rust
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
    ```

And we get the following output -

=== "Rust"

    ```rust
    version_5_a ran with mean 10.116 microseconds and variance 1.1695418 microseconds squared
    version_5_b ran with mean 10.315 microseconds and variance 0.94678104 microseconds squared
    version_5_c ran with mean 1046.928 microseconds and variance 1914.6855 microseconds squared
    version_5_d ran with mean 1.217 microseconds and variance 0.17091024 microseconds squared
    ```

If we simulate an uneven workload across executions by making a more erratic function for ```a()```. A random number
is generated for each call to the ```a_erratic()```, 1 out of 60 calls will result in an execution time which should
be approximately be 100 times slower. We get the following results -

=== "Rust"

    ```rust
    version_6_a ran with mean 23.2655 microseconds and variance 13236.803 microseconds squared
    version_6_b ran with mean 10.359 microseconds and variance 0.77912927 microseconds squared
    version_6_c ran with mean 1066.4165 microseconds and variance 2758.1538 microseconds squared
    version_6_d ran with mean 1.0035 microseconds and variance 0.0034878778 microseconds squared
    ```

Clearly, we should be getting the hint that something is irregular about the execution of our function
```a_erratic()```. Another thing we could do, if these functions took input data, would be to benchmark
the execution times across various input data sizes and graph it. I won't do it in this function, it is a
bit more involved. But it is how the graphs were generated in the framework for computational graphs.
You can find the code for making the graph based on vectors of performance measurements in
[```m1_memory_hierarchies::code::computational_graphs::src::shared::benchmark_plot.rs```][1].

<figure markdown>
![Image](../figures/graphs_size_benchmark.png){ width="600" }
<figcaption>
An example of a timing plot, comparing different implementations across different input sizes.
</figcaption>
</figure>

This way of benchmarking different implementations won't help us find bottlenecks in our specific system.
To do that we can either keep timing and printing, going deeper and deeper, or get a profiler to measure
which functions in our code are the biggest hot spots. These profilers are system and hardware dependent, so I will
suggest ones you can try out in the next section.

[0]: https://github.com/absorensen/the-guide/tree/main/m5_optimization/code/measuring_performance
[1]: https://github.com/absorensen/the-guide/blob/main/m1_memory_hierarchies/code/computational_graphs/src/shared/benchmark_plot.rs
