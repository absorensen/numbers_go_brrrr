use std::{mem, time::{Instant, Duration}};

use rand::{rngs::ThreadRng, Rng};

struct NaiveJaggedArray {
    data: Vec<Vec<f32>>,
    total_elements: usize,
}

impl NaiveJaggedArray {
    fn new(row_lengths: Vec<usize>) -> Self {
        let mut data: Vec<Vec<f32>> = Vec::<Vec<f32>>::new();
        let mut total_elements: usize = 0;
        for (row_index, length) in row_lengths.into_iter().enumerate() {
            data.push(vec![row_index as f32; length]);
            total_elements += length;
        }

        NaiveJaggedArray { data, total_elements }
    }

    fn sum(&self) -> f32 {
        let mut sum: f32 = 0.0;
        for row in &self.data {
            for column in row {
                sum += *column;
            }
        }

        sum
    }

    #[inline(always)]
    fn random_access(&self, row_index: usize, column_index: usize) -> Option<f32> {
        if self.data.len() <= row_index { return None; };
        if self.data[row_index].len() <= column_index { return None; };

        Some(self.data[row_index][column_index])
    }

    fn get_memory_size(&self) -> usize {
        mem::size_of::<f32>() * self.total_elements + 
        mem::size_of::<Vec<f32>>() * self.data.len() + 
        mem::size_of::<Vec<Vec<f32>>>()
    }
}

struct JaggedArrayAuxLengths {
    data: Vec<f32>,
    lengths: Vec<usize>,
    max_row_length: usize,
}

impl JaggedArrayAuxLengths {
    fn new(row_count: usize, row_lengths: Vec<usize>) -> Self {
        let max_row_length: usize = *row_lengths.iter().max().unwrap();
        let mut data: Vec<f32> = vec![0.0; max_row_length * row_count];

        for row_index in 0..row_count {
            let linear_row_index: usize = row_index * max_row_length;
            for column_index in 0..row_lengths[row_index] {
                data[linear_row_index + column_index] = row_index as f32;
            }
        }

        JaggedArrayAuxLengths { data, lengths: row_lengths, max_row_length } 
    }

    fn sum(&self) -> f32 {
        let mut sum: f32 = 0.0;
        
        let mut current_row_index: usize = 0;
        let mut current_index: usize = 0;
        for length in &self.lengths {
            for _ in 0..*length {
                sum += self.data[current_index];
            }
            current_row_index += 1;
            current_index = current_row_index * self.max_row_length;
        }

        sum
    }

    #[inline(always)]
    fn random_access(&self, row_index: usize, column_index: usize) -> Option<f32> {
        if self.lengths.len() <= row_index { return None; };
        if self.lengths[row_index] <= column_index { return None; };

        Some(self.data[row_index * self.max_row_length + column_index])
    }

    fn get_memory_size(&self) -> usize {
        mem::size_of::<f32>() * self.data.len() + 
        mem::size_of::<Vec<f32>>() + 
        mem::size_of::<usize>() * self.lengths.len() 
        + mem::size_of::<Vec<usize>>()
    }
}

struct ConstrainedJaggedArray {
    data: Vec<f32>,
    max_row_length: usize,
    row_count: usize,
}

impl ConstrainedJaggedArray {
    fn new(row_count: usize, row_lengths: Vec<usize>) -> Self {
        let max_row_length: usize = *row_lengths.iter().max().unwrap() + 1;
        let mut data: Vec<f32> = vec![0.0; max_row_length * row_count];

        for row_index in 0..row_count {
            let linear_row_index: usize = row_index * max_row_length;
            data[linear_row_index] = row_lengths[row_index] as f32;
            for column_index in 1..row_lengths[row_index]+1 {
                data[linear_row_index + column_index] = row_index as f32;
            }
        }

        ConstrainedJaggedArray { data, max_row_length, row_count}
    }

    fn sum(&self) -> f32 {
        let mut sum: f32 = 0.0;
        for row_index in 0..self.row_count {
            let row_index: usize = self.max_row_length * row_index;
            for column_index in 0..(self.data[row_index] as usize) {
                sum += self.data[row_index + 1 + column_index];
            }
        }

        sum
    }

    #[inline(always)]
    fn random_access(&self, row_index: usize, column_index: usize) -> Option<f32> {
        if self.row_count <= row_index { return None; };
        if self.data[row_index * self.max_row_length] as usize <= column_index { return None; };

        Some(self.data[row_index * self.max_row_length + column_index])
    }

    fn get_memory_size(&self) -> usize {
        mem::size_of::<f32>() * self.data.len() + 
        mem::size_of::<Vec<f32>>() +
        mem::size_of::<usize>() * 2
    }
}

struct CompactedJaggedArray {
    data: Vec<f32>,
    row_count: usize,
}

impl CompactedJaggedArray {
    fn new(row_count: usize, row_lengths: Vec<usize>) -> Self {
        let data_count: usize = row_lengths.iter().sum::<usize>() + row_count;
        let mut data: Vec<f32> = vec![0.0; data_count];

        let mut current_index: usize = 0;
        let mut current_row_index: usize = 0;
        for row_length in &row_lengths {
            data[current_index] = *row_length as f32;
            current_index += 1;
            for _ in 0..*row_length {
                data[current_index] = current_row_index as f32;
                current_index += 1;
            }
            current_row_index += 1;
        }

        CompactedJaggedArray { data, row_count: row_lengths.len() }
    }

    fn sum(&self) -> f32 {
        let mut sum: f32 = 0.0;
        let mut current_index: usize = 0;
        while current_index < self.data.len() {
            let current_row_length: usize = self.data[current_index] as usize;
            current_index += 1;
            for _ in 0..current_row_length {
                sum += self.data[current_index];
                current_index += 1;
            }
        }

        sum
    }

    #[inline(always)]
    fn random_access(&self, row_index: usize, column_index: usize) -> Option<f32> {
        if self.row_count <= row_index { return None; };
        
        let mut current_index: usize = 0;
        let mut current_row: usize = 0;
        while current_index < self.data.len() {
            while current_row < row_index {
                current_index += self.data[current_index] as usize;
                current_row += 1;
            }

            if self.data[current_index] as usize <= column_index {
                return None;
            }

            return Some(self.data[current_index + column_index]);
        }

        None
    }

    fn get_memory_size(&self) -> usize {
        mem::size_of::<f32>() * self.data.len() + 
        mem::size_of::<Vec<f32>>()
    }
}

struct CompactedJaggedArrayAuxRowStart {
    data: Vec<f32>,
    row_starts: Vec<usize>,
}

impl CompactedJaggedArrayAuxRowStart {
    fn new(row_count: usize, row_lengths: Vec<usize>) -> Self {
        let data_count: usize = row_lengths.iter().sum::<usize>() + row_count;
        let mut data: Vec<f32> = vec![0.0; data_count];
        let mut row_starts: Vec<usize> = vec![0; row_lengths.len() + 1];

        let mut current_index: usize = 0;
        let mut current_row_index: usize = 0;
        let mut row_start: usize = 0;
        for row_length in row_lengths {
            row_starts[current_row_index] = row_start;
            for _ in 0..row_length {
                data[current_index] = current_row_index as f32;
                current_index += 1;
            }
            current_row_index += 1;
            row_start += row_length;
        }
        row_starts[current_row_index] = row_start;

        CompactedJaggedArrayAuxRowStart { data, row_starts}
    }

    fn sum(&self) -> f32 {
        self.data.iter().sum()
    }

    #[inline(always)]
    fn random_access(&self, row_index: usize, column_index: usize) -> Option<f32> {
        if self.row_starts.len() - 1 <= row_index { return None; };
        let row_length: usize = self.row_starts[row_index + 1] - self.row_starts[row_index]; 
        if row_length <= column_index { return None; };

        Some(self.data[self.row_starts[row_index] + column_index])
    }

    fn get_memory_size(&self) -> usize {
        mem::size_of::<f32>() * self.data.len() + 
        mem::size_of::<Vec<f32>>() + 
        mem::size_of::<usize>() * self.row_starts.len() 
        + mem::size_of::<Vec<usize>>()
    }
}

fn execute_test(iteration_count: usize, row_count: usize, max_row_length: usize, row_lengths: Vec<usize>, random_access_count: usize, run_expensive_tests: bool) -> f32 {
    println!("Running test with {} row count, {} max row length, {} iterations, {} random accesses", row_count, max_row_length, iteration_count, random_access_count);
    
    let mut sum: f32 = 0.0;
    let mut rng: ThreadRng = rand::thread_rng();

    
    // Introduce scope to drop the vectors once they are done
    {
        let naive_jagged_array: NaiveJaggedArray = NaiveJaggedArray::new(row_lengths.clone());
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            sum += naive_jagged_array.sum();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for NaiveJaggedArray sum test taking {} bytes of memory", elapsed_time.as_millis() as f64, naive_jagged_array.get_memory_size());

        let mut time_sum: Duration = Duration::new(0, 0);
        // Generating the random indices might be just as expensive as making the accesses so we do this in bulk
        // outside the timing.
        let mut random_indices: Vec<(usize, usize)> = (0..random_access_count).into_iter().map(|_| (rng.gen_range(0..row_count), rng.gen_range(0..max_row_length))).collect();
        for _ in 0..iteration_count {
            for indices in &mut random_indices {
                indices.0 = rng.gen_range(0..row_count);
                indices.1 = rng.gen_range(0..max_row_length);
            }
            let now: Instant = Instant::now();
            for (row_index, column_index) in &random_indices {
                if let Some(value) = naive_jagged_array.random_access(*row_index, *column_index) {
                    sum += value;
                }
            }
            let elapsed_time: Duration = now.elapsed();
            time_sum += elapsed_time;
        }
        println!("{} ms for NaiveJaggedArray random access test taking {} bytes of memory", time_sum.as_millis() as f64, naive_jagged_array.get_memory_size());
    }

    // Introduce scope to drop the vectors once they are done
    {
        let jagged_array_aux_lengths: JaggedArrayAuxLengths = JaggedArrayAuxLengths::new(row_count, row_lengths.clone());
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            sum += jagged_array_aux_lengths.sum();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for JaggedArrayAuxLengths sum test taking {} bytes of memory", elapsed_time.as_millis() as f64, jagged_array_aux_lengths.get_memory_size());

        let mut time_sum: Duration = Duration::new(0, 0);
        // Generating the random indices might be just as expensive as making the accesses so we do this in bulk
        // outside the timing.
        let mut random_indices: Vec<(usize, usize)> = (0..random_access_count).into_iter().map(|_| (rng.gen_range(0..row_count), rng.gen_range(0..max_row_length))).collect();
        for _ in 0..iteration_count {
            for indices in &mut random_indices {
                indices.0 = rng.gen_range(0..row_count);
                indices.1 = rng.gen_range(0..max_row_length);
            }
            let now: Instant = Instant::now();
            for (row_index, column_index) in &random_indices {
                if let Some(value) = jagged_array_aux_lengths.random_access(*row_index, *column_index) {
                    sum += value;
                }
            }
            let elapsed_time: Duration = now.elapsed();
            time_sum += elapsed_time;
        }
        println!("{} ms for JaggedArrayAuxLengths random access test taking {} bytes of memory", time_sum.as_millis() as f64, jagged_array_aux_lengths.get_memory_size());
    }


    // Introduce scope to drop the vectors once they are done
    {
        let constrained_jagged_array: ConstrainedJaggedArray = ConstrainedJaggedArray::new(row_count, row_lengths.clone());
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            sum += constrained_jagged_array.sum();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for ConstrainedJaggedArray sum test taking {} bytes of memory", elapsed_time.as_millis() as f64, constrained_jagged_array.get_memory_size());

        let mut time_sum: Duration = Duration::new(0, 0);
        // Generating the random indices might be just as expensive as making the accesses so we do this in bulk
        // outside the timing.
        let mut random_indices: Vec<(usize, usize)> = (0..random_access_count).into_iter().map(|_| (rng.gen_range(0..row_count), rng.gen_range(0..max_row_length))).collect();
        for _ in 0..iteration_count {
            for indices in &mut random_indices {
                indices.0 = rng.gen_range(0..row_count);
                indices.1 = rng.gen_range(0..max_row_length);
            }
            let now: Instant = Instant::now();
            for (row_index, column_index) in &random_indices {
                if let Some(value) = constrained_jagged_array.random_access(*row_index, *column_index) {
                    sum += value;
                }
            }
            let elapsed_time: Duration = now.elapsed();
            time_sum += elapsed_time;
        }
        println!("{} ms for ConstrainedJaggedArray random access test taking {} bytes of memory", time_sum.as_millis() as f64, constrained_jagged_array.get_memory_size());
    }

    
    // Introduce scope to drop the vectors once they are done
    {
        let compacted_jagged_array: CompactedJaggedArray = CompactedJaggedArray::new(row_count, row_lengths.clone());
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            sum += compacted_jagged_array.sum();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for CompactedJaggedArray sum test taking {} bytes of memory", elapsed_time.as_millis() as f64, compacted_jagged_array.get_memory_size());

        if run_expensive_tests {
            let mut time_sum: Duration = Duration::new(0, 0);
            // Generating the random indices might be just as expensive as making the accesses so we do this in bulk
            // outside the timing.
            let mut random_indices: Vec<(usize, usize)> = (0..random_access_count).into_iter().map(|_| (rng.gen_range(0..row_count), rng.gen_range(0..max_row_length))).collect();
            for _ in 0..iteration_count {
                for indices in &mut random_indices {
                    indices.0 = rng.gen_range(0..row_count);
                    indices.1 = rng.gen_range(0..max_row_length);
                }
                let now: Instant = Instant::now();
                for (row_index, column_index) in &random_indices {
                    if let Some(value) = compacted_jagged_array.random_access(*row_index, *column_index) {
                        sum += value;
                    }
                }
                let elapsed_time: Duration = now.elapsed();
                time_sum += elapsed_time;
            }
            println!("{} ms for CompactedJaggedArray random access test taking {} bytes of memory", time_sum.as_millis() as f64, compacted_jagged_array.get_memory_size());
        } else {
            println!("Didn't run random access test for CompactedJaggedArray because it was too expensive!");
        }
    }

    // Introduce scope to drop the vectors once they are done
    {
        let compacted_jagged_array_aux_row_start: CompactedJaggedArrayAuxRowStart = CompactedJaggedArrayAuxRowStart::new(row_count, row_lengths.clone());
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            sum += compacted_jagged_array_aux_row_start.sum();
        }
        let elapsed_time: Duration = now.elapsed();
        println!("{} ms for CompactedJaggedArrayAuxRowStart sum test taking {} bytes of memory", elapsed_time.as_millis() as f64, compacted_jagged_array_aux_row_start.get_memory_size());

        let mut time_sum: Duration = Duration::new(0, 0);
        // Generating the random indices might be just as expensive as making the accesses so we do this in bulk
        // outside the timing.
        let mut random_indices: Vec<(usize, usize)> = (0..random_access_count).into_iter().map(|_| (rng.gen_range(0..row_count), rng.gen_range(0..max_row_length))).collect();
        for _ in 0..iteration_count {
            for indices in &mut random_indices {
                indices.0 = rng.gen_range(0..row_count);
                indices.1 = rng.gen_range(0..max_row_length);
            }
            let now: Instant = Instant::now();
            for (row_index, column_index) in &random_indices {
                if let Some(value) = compacted_jagged_array_aux_row_start.random_access(*row_index, *column_index) {
                    sum += value;
                }
            }
            let elapsed_time: Duration = now.elapsed();
            time_sum += elapsed_time;
        }
        println!("{} ms for CompactedJaggedArrayAuxRowStart random access test taking {} bytes of memory", time_sum.as_millis() as f64, compacted_jagged_array_aux_row_start.get_memory_size());
    }

    println!("");
    println!("");

    sum
}

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut sums: f32 = 0.0;


    let iteration_count: usize = 1_000_000;
    let row_count: usize = 10;
    let max_row_length: usize = 10;
    let row_lengths: Vec<usize> = (0..row_count).into_iter().map(|_| rng.gen_range(0..max_row_length)).collect();
    let random_access_count: usize = row_count * max_row_length / 10;
    let run_expensive_tests: bool = true;

    sums += execute_test(iteration_count, row_count, max_row_length, row_lengths, random_access_count, run_expensive_tests);


    let iteration_count: usize = 100_000;
    let row_count: usize = 100;
    let max_row_length: usize = 100;
    let row_lengths: Vec<usize> = (0..row_count).into_iter().map(|_| rng.gen_range(0..max_row_length)).collect();
    let random_access_count: usize = row_count * max_row_length / 10;
    let run_expensive_tests: bool = true;

    sums += execute_test(iteration_count, row_count, max_row_length, row_lengths, random_access_count, run_expensive_tests);


    let iteration_count: usize = 1_000;
    let row_count: usize = 1000;
    let max_row_length: usize = 1000;
    let row_lengths: Vec<usize> = (0..row_count).into_iter().map(|_| rng.gen_range(0..max_row_length)).collect();
    let random_access_count: usize = row_count * max_row_length / 10;
    let run_expensive_tests: bool = false;

    sums += execute_test(iteration_count, row_count, max_row_length, row_lengths, random_access_count, run_expensive_tests);


    let iteration_count: usize = 100;
    let row_count: usize = 10000;
    let max_row_length: usize = 10000;
    let row_lengths: Vec<usize> = (0..row_count).into_iter().map(|_| rng.gen_range(0..max_row_length)).collect();
    let random_access_count: usize = row_count * max_row_length / 100;
    let run_expensive_tests: bool = false;
    
    sums += execute_test(iteration_count, row_count, max_row_length, row_lengths, random_access_count, run_expensive_tests);


    let iteration_count: usize = 1;
    let row_count: usize = 100000;
    let max_row_length: usize = 100000;
    let row_lengths: Vec<usize> = (0..row_count).into_iter().map(|_| rng.gen_range(0..max_row_length)).collect();
    let random_access_count: usize = row_count * max_row_length / 1000;
    let run_expensive_tests: bool = false;

    sums += execute_test(iteration_count, row_count, max_row_length, row_lengths, random_access_count, run_expensive_tests);

    println!("Sum was: {}", sums);
}
