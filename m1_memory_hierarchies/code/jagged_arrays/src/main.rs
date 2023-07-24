use std::mem;

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


fn main() {
    println!("Hello, world!");
}
