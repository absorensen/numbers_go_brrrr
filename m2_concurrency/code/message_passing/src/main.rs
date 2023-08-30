use std::{sync::mpsc, thread::{self}, time::Duration};
use rand::prelude::*;

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

fn main() {
    let max_work: usize = 100;
    let master_wait_time: u64 = 200;
    let worker_wait_time: u64 = 200;

    let (work_transmitter, work_receiver) = mpsc::channel::<Vec<f32>>();
    let (result_transmitter, result_receiver) = mpsc::channel::<Vec<f32>>();

    let task: Vec<f32> = vec![0.1];
    work_transmitter.send(task);

    let task: Vec<f32> = vec![0.1, 0.2];
    work_transmitter.send(task);

    let task: Vec<f32> = vec![0.1, 0.2, 0.3];
    work_transmitter.send(task);

    let task: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4];
    work_transmitter.send(task);

    thread::spawn(move || {
        let mut work_performed_count: usize = 0;
        loop {
            match work_receiver.try_recv() {
                Ok(mut work) => {
                    map_function(&mut work);
                    result_transmitter.send(work);
        
                    work_performed_count += 1;
        
                    if max_work < work_performed_count {
                        return;
                    }
                },
                Err(_) => {
                    println!("Tried to get some work to do, but none was ready in the channel!");
                }
            }

            // This represents the thread doing other work.
            thread::sleep(Duration::from_millis(worker_wait_time));
        }
    });

    // We could also just move this to its own thread, but then we would
    // need to keep the main thread alive to keep the program running.
    let mut rng: ThreadRng = thread_rng();
    let mut work_sent_count: usize = 0;
    loop {
        match result_receiver.try_recv(){
            Ok(result) => {
                println!("Received result {:?}", result);

                let new_length: usize = rng.gen_range(1..5);
                let new_task: Vec<f32> = (0..new_length).map(|_| rng.gen_range(-1.0..1.0)).collect();
        
                work_transmitter.send(new_task);
        
                work_sent_count += 1;
        
                if max_work < work_sent_count {
                    return;
                }
            },
            Err(_) => {
                println!("Tried to get results from receiver, but there were none ready.")
            }
        }
        
        // This represents the thread doing other work.
        thread::sleep(Duration::from_millis(master_wait_time));
    }

}
