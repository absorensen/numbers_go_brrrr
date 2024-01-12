use std::{thread, time, str::FromStr};

use rand::Rng;

pub struct Worker {
    pub out_for_walk: bool,
    pub in_a_union: String,
    pub store_credits: i16,
    pub real_money: f32,
}

impl Worker {
    fn abduct_random_worker() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            out_for_walk: rng.gen_bool(0.001),
            in_a_union: String::from_str("No").unwrap(),
            store_credits: rng.gen_range(0..8), 
            real_money: rng.gen_range(-435.0..0.0)
        }
    }

    pub fn generate_workers(worker_count: usize) -> Vec<Self>{
        let mut workers: Vec<Self> = Vec::<Self>::new();
        for _ in 0..worker_count {
            workers.push(Self::abduct_random_worker());
        }

        workers
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        let joins_union = rng.gen_bool(0.2);
        if joins_union {
            self.in_a_union = String::from_str("Yes").unwrap();
        }
        self.store_credits -= rng.gen_range(0..4);
        self.real_money -= rng.gen_range(0.0..5.0);
        self.out_for_walk = rng.gen_bool(0.01);
    }

}

pub fn strike_down_the_unions(workers: &mut Vec<Worker>) {
    let yes: String = String::from_str("Yes").unwrap();
    for worker in workers {
        if worker.in_a_union == yes {
            worker.in_a_union = String::from_str("No").unwrap();
        }
    }
}

pub fn update_workers(workers:&mut Vec<Worker>) {
    for worker in workers {
        worker.update();
    }
}

pub fn let_the_workers_out_for_a_walk(max_wait_time: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let worker_walk_time: u32 = rng.gen_range(0..(max_wait_time*2));

    let wait_time = worker_walk_time.min(max_wait_time);
    let wait_time = time::Duration::from_millis(wait_time as u64);
    thread::sleep(wait_time);

    (max_wait_time - worker_walk_time).max(0)
}

pub fn pay_workers_in_store_credits(amount: u8, workers: &mut Vec<Worker>) {
    for worker in workers {
        worker.store_credits += amount as i16;
    }
}