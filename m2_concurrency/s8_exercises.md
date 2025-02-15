# 👨🏼‍💻 Exercises
These exercises aren't necessarily about getting the best performance, but about trying combinations
of the concepts introduced in the module. It can still be interesting to time and compare the different strategies
to get a better feel for which strategies function best with which problems.

## Parallel Sums and Histograms
For this series of exercises we'll look at using some parallelization mechanisms
for summing a list of numbers or creating a histogram of the number of occurences
of different integer values.

First we'll create a new project which can contain all of the exercises.
Create a new Rust project using ```cargo new parallel_sums_and_histograms```.

=== "Cargo.toml"

    ```rust
    [dependencies]
    rayon = "*"
    rand = "*"
    ```

=== "main.rs"

    ```rust
    use rand::{Rng, rngs::ThreadRng};
    use std::sync::{atomic::AtomicU64, Mutex, MutexGuard, mpsc::channel};
    use rayon::prelude::*;

    const BIN_COUNT: usize = 16;
    const THREAD_COUNT: usize = 4;

    fn main() {
        let mut rng: ThreadRng = rand::thread_rng();
        let chunk_size: usize = 4;
        let data_count: usize = 24;
        let data: Vec<u32> = (0..data_count).into_iter().map(|_| rng.gen_range(0..BIN_COUNT) as u32 ).collect();

        println!("Correct sum is: {}", data.iter().sum::<u32>());
        parallel_sum(&data);
        parallel_sum_with_atomic(&data, chunk_size);
        parallel_histogram_with_atomics(&data, chunk_size);
        parallel_histogram_with_lock(&data, chunk_size);
        parallel_histogram_with_threads_and_channels(&data);
    }
    ```

### m2::e0 - Parallel Sum
Create the parallel sum function. It should take in the data vector and sum the numbers using Rayon. Do it as
simply as possible.

??? note "Hints"

    .par_iter().sum() is almost all you need.

### m2::e1 - Parallel Sum with Atomic
Now, create and fill out the parallel sum with atomic function. Instead of using the .sum() function, try and create
an atomic variable outside of the iterator for summing the data. You can use either .par_iter() or .par_chunks().

??? note "Hints"

    To solve this in very few lines, use
    [.par_chunks(chunk_size)](https://docs.rs/rayon/latest/rayon/slice/trait.ParallelSlice.html#method.par_chunks)
    , [.for_each()](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.for_each)
    , [AtomicU64](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU64.html)
    and [.fetch_add()](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU64.html#method.fetch_add)
    with a relaxed ordering.

Why might you use .par_chunks instead()?

??? success "Answer"
    Summing as many of the values as possible locally mitigates the contention for the atomic sum variable.

### m2::e2 - Parallel Histogram with Atomics
For the parallel histogram with atomics function you should now create an array of atomics instead. For each
value add to the count for the bin.

??? note "Hints - Atomic Array"

    You can create a statically sized array of atomics using the following snippet -

    === "Rust"

        ```rust
        let e2: [AtomicU64; BIN_COUNT] = [0_u64; BIN_COUNT].map(|x| AtomicU64::new(x));
        ```

??? note "Hints - Contention"

    If you want to mitigate contention in the atomic array, make a chunk local array to
    compute a local histogram before adding the other elements.

    You can also lower contention by checking whether you have a 0 in the histogram
    element you are about to atomically add.

### m2::e3 - Parallel Histogram with Locks
Now instead of each thread accessing an individual element at a time, try using an array of bin values with the
entire array wrapped in a Mutex (lock). Why not use a RwLock instead of a Mutex?

### m2::e4 - Parallel Histogram with Channels and Threads
In this exercise a number of threads will receive a subset of the input data, compute their own subhistrogram,
then send that subhistogram to another thread for accumulation into a final histogram.

* Partition the data vector into ```thread_count``` number of slices.
* Create a channel with a sender/receiver pair.
* Move the slices and senders (by cloning the sender) to ```thread_count``` number of threads.
* The main thread will block on the receiver and take all incoming subhistograms. Once finished,
it should print its histogram. But how do you know it's finished?

??? note "Hints"

    * You can partition the input data into [&[u32]](https://doc.rust-lang.org/rust-by-example/primitives/array.html)
    slices, by using [.split_at()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_at).
    * For creating a channel, check out the
    [std::sync::mpsc::channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html).
    * Create threads using [thread::spawn](https://doc.rust-lang.org/std/thread/fn.spawn.html).
    * You can check for all senders having been dropped, by handling the receiving of ```Err(_)``` in your accumulator.
    * Make sure to join your threads.
