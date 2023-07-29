use std::{collections::HashMap, time::{Instant, Duration}};

use rand::{thread_rng, Rng};

fn main() {
    let element_iteration_counts: [(usize, usize); 4] = [(1000, 100000), (10000, 10000), (100000, 1000), (1000000, 100)];
    let mut rng = thread_rng();

    // Insert test
    for (element_count, iteration_count) in element_iteration_counts {
        println!("Commencing test of {} elements for {} iterations!", element_count, iteration_count);
        let mut values: Vec<i64> = Vec::<i64>::new();
        let mut string_keys: Vec<String> = Vec::<String>::new();

        for _ in 0..element_count {
            let value: i64 = rng.gen::<i64>();
            string_keys.push(value.to_string());
            values.push(value);
        }

        // Create the string_map and insert all of the values
        let mut string_map: HashMap<String, i64> = HashMap::<String, i64>::new();

        let now: Instant = Instant::now();
        for (key, value) in string_keys.iter().zip(values.iter()) {
            // string_map takes ownership of the key, so we need to
            // copy, which will be quite inefficient
            string_map.insert(key.clone(), *value);
        }
        let elapsed_time: Duration = now.elapsed();
        println!("Took {} microseconds to insert {} elements into string_map.", elapsed_time.as_micros() as f64, element_count);

        
        // Measure read and update time for string_map
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            for key in &string_keys {
                *string_map.get_mut(key).unwrap() += 1;
            }
        }
        let elapsed_time: Duration = now.elapsed();
        println!("Took {} ms to read {} elements from string_map for {} iterations.", elapsed_time.as_millis() as f64, element_count, iteration_count);


        // Create the int_map and insert all of the values
        let mut int_map: HashMap<i64, i64> = HashMap::<i64, i64>::new();
        let now: Instant = Instant::now();
        for value in &values {
            int_map.insert(*value, *value);
        }
        let elapsed_time: Duration = now.elapsed();
        println!("Took {} microseconds to insert {} elements into int_map.", elapsed_time.as_micros() as f64, element_count);


        // Measure read and update time for int_map
        let now: Instant = Instant::now();
        for _ in 0..iteration_count {
            for key in &values {
                *int_map.get_mut(key).unwrap() += 1;
            }
        }
        let elapsed_time: Duration = now.elapsed();
        println!("Took {} ms to read {} elements from int_map for {} iterations.", elapsed_time.as_millis() as f64, element_count, iteration_count);

        println!("");
    }

}
