mod model_loading;
mod point_cloud_processing;
mod pandemonium_machine;
mod kettles;
mod workers;
mod rainbow_curvature;
mod pretty_screensaver;
mod geometry_machinery;

use std::{time::Instant, collections::HashMap};

use geometry_machinery::GeometryMachinery;
use model_loading::load_a_bunny;
use pandemonium_machine::generate_pandemonium_particles;
use point_cloud_processing::point_sample_model;
use rand::{thread_rng, Rng};
use tobj::Model;
use ultraviolet::Vec3;
use workers::Worker;

use crate::{kettles::put_the_kettle_on, workers::{let_the_workers_out_for_a_walk, pay_workers_in_store_credits, update_workers, strike_down_the_unions}, point_cloud_processing::displace_point_cloud_relative_to_center, rainbow_curvature::calculate_the_curvature_of_the_rainbow, pretty_screensaver::PrettyScreensaver, geometry_machinery::work_on_geometry_machinery};

struct Timings {
    times: HashMap<String, (u128, u128)>,
}

impl Timings {
    pub fn new() -> Self {
        Timings { times: HashMap::<String, (u128, u128)>::new() }
    }

    pub fn print_miliseconds(&self) {
        for (key, value) in &self.times {
            println!("{}: {} ms", key, (*value).0 as f64 / 1_000_000.0 / (*value).1 as f64);
        }
    }

    pub fn create_or_add_to_entry(&mut self, name: &str, value: u128) {
        let name: String = name.to_string();
        if self.times.contains_key(&name) {
            let pair = self.times.get_mut(&name).unwrap();
            pair.0 += value;
            pair.1 += 1;
        } else {
            self.times.insert(name, (value, 1));
        }
    }
}

fn preprocessing(total_time_in_nano: &mut Timings, _function_time_in_nano: &mut Timings) {
    let preprocessing_time_start = Instant::now();

    let preprocessing_time_stop = Instant::now();
    total_time_in_nano.create_or_add_to_entry("Preprocessing", (preprocessing_time_stop-preprocessing_time_start).as_nanos());
}

fn load_bunny(function_time_in_nano: &mut Timings, path_to_bunny: &String) -> Vec<Model> {
    let start = Instant::now();
    let models = load_a_bunny(path_to_bunny);
    let stop = Instant::now();
    function_time_in_nano.create_or_add_to_entry("load_a_bunny", (stop - start).as_nanos());

    models
}

fn point_sample_bunny(function_time_in_nano: &mut Timings, models: &Vec<Model>, point_sampling_factor: f32) -> Vec<Vec3> {
    let start = Instant::now();
    let point_cloud = point_sample_model(&models[0].mesh, point_sampling_factor);
    let stop = Instant::now();
    function_time_in_nano.create_or_add_to_entry("point_sample_model", (stop - start).as_nanos());
    
    point_cloud
}

fn pandemonium(function_time_in_nano: &mut Timings, point_cloud: &Vec<Vec3>) -> Vec<Vec3> {
    let start = Instant::now();
    let particles = generate_pandemonium_particles(&point_cloud);
    let stop = Instant::now();
    function_time_in_nano.create_or_add_to_entry("pandemonium_particles", (stop - start).as_nanos());

    particles
}

fn kettle(function_time_in_nano: &mut Timings, target_temperature: f32, max_time_steps: u32) -> bool {
    // PUT THE NON-DETERMINISTIC KETTLE ON
    let start = Instant::now();
    let kettle_done_in_time: bool = put_the_kettle_on(target_temperature, max_time_steps);
    let stop = Instant::now();
    function_time_in_nano.create_or_add_to_entry("kettle_done_in_time", (stop - start).as_nanos());

    kettle_done_in_time
}

fn handle_workers(function_time_in_nano: &mut Timings, workers: &mut Vec<Worker>, max_wait_time: u32, worker_count: usize) {
    // LET THE WORKERS OUT FOR A WALK AND WAIT FOR A RANDOM AMOUNT OF TIME UNTIL THEY COMES BACK
    // IF THEY DON'T COME BACK IN TIME, DON'T PAY THEM
    let start = Instant::now();
    let workers_returned_in_time: u32 = let_the_workers_out_for_a_walk(max_wait_time);
    let stop = Instant::now();
    function_time_in_nano.create_or_add_to_entry("let_the_workers_out_for_a_walk", (stop - start).as_nanos());
    // If they come back in time, pay them with store credits
    if 0 < workers_returned_in_time {
        if workers.len() == 0 {
            *workers = Worker::generate_workers(worker_count)
        }
        let start = Instant::now();
        pay_workers_in_store_credits((workers_returned_in_time >> 4) as u8, workers);
        let stop = Instant::now();
        function_time_in_nano.create_or_add_to_entry("pay_workers_in_store_credits", (stop - start).as_nanos());
    }
}

fn handle_union(function_time_in_nano: &mut Timings, workers: &mut Vec<Worker>) {
    let start = Instant::now();
    strike_down_the_unions(workers);
    let stop = Instant::now();
    function_time_in_nano.create_or_add_to_entry("strike_down_the_unions", (stop - start).as_nanos());
}

fn manipulate_point_cloud(function_time_in_nano: &mut Timings, point_cloud: &mut Vec<Vec3>) {
    // FIND THE CENTER OF THE POINT CLOUD AND MOVE ALL THE POINTS TOWARDS AND AWAY FROM THE CENTER
    let start = Instant::now();
    displace_point_cloud_relative_to_center(0.2, point_cloud);
    let stop = Instant::now();
    function_time_in_nano.create_or_add_to_entry("displace_point_cloud_relative_to_center", (stop - start).as_nanos());    
}

fn update_screensaver(function_time_in_nano: &mut Timings, screensaver: &mut PrettyScreensaver) {
    let start = Instant::now();
    screensaver.update();
    let stop = Instant::now();
    function_time_in_nano.create_or_add_to_entry("screensaver.update", (stop - start).as_nanos());
}

fn build_geometry_machine(function_time_in_nano: &mut Timings, geometry_machine_thread_count: u64, work_amount_per_gizmo: u64, gizmos_per_machine: u64) -> GeometryMachinery {
    let start = Instant::now();
    let geometry_machinery = work_on_geometry_machinery(geometry_machine_thread_count, work_amount_per_gizmo, gizmos_per_machine);
    let stop = Instant::now();
    function_time_in_nano.create_or_add_to_entry("work_on_geometry_machinery", (stop - start).as_nanos());

    geometry_machinery
}

// Yell at them to get hardcore or get fired
fn yell_at_workers(function_time_in_nano: &mut Timings, workers: &mut Vec<Worker>) {
    let start = Instant::now();
    update_workers(workers);
    let stop = Instant::now();
    function_time_in_nano.create_or_add_to_entry("update_workers", (stop - start).as_nanos());
}

fn curvature_of_the_rainbow(
    function_time_in_nano: &mut Timings,
    kettle_done_in_time: bool,
    workers: &Vec<Worker>,
    point_cloud: &Vec<Vec3>,
    particles: &Vec<Vec3>,
    geometry_machinery: &GeometryMachinery,
) {
    let mut rng = thread_rng();
    let at_point: Vec3 = Vec3::new(rng.gen(), rng.gen(), rng.gen());
    let start = Instant::now();
    let curvature = calculate_the_curvature_of_the_rainbow(&at_point, kettle_done_in_time, workers, point_cloud, particles, geometry_machinery);
    let stop = Instant::now();
    function_time_in_nano.create_or_add_to_entry("calculate_the_curvature_of_the_rainbow", (stop - start).as_nanos());
    println!("Curvature of the Rainbow at {:?} was {:?}", at_point, curvature);
}

fn main() {
    // DON'T CHANGE THESE
    let point_sampling_factor: f32 = 1000.0;
    let target_temperature = 9432.3;
    let max_time_steps = 1_000_000_000;
    let max_wait_time = 1_000;
    let worker_count: usize = 13123170;
    let geometry_machine_thread_count = 1000;
    let work_amount_per_gizmo = 1631245;
    let gizmos_per_machine = 11224152;
    // DON'T CHANGE THESE

    let mut workers: Vec<Worker> = Vec::<Worker>::new();
    let mut total_time_in_nano: Timings = Timings::new();
    let mut function_time_in_nano: Timings = Timings::new();
    let path_to_bunny: String = "./resources/bunny.obj".to_string();
    let mut screensaver = PrettyScreensaver::new();
    let loop_count = 1;
    
    preprocessing(&mut total_time_in_nano, &mut function_time_in_nano);

    let loop_time_start = Instant::now();
    for _ in 0..loop_count {
        let models = load_bunny(&mut function_time_in_nano, &path_to_bunny);
        
        let mut point_cloud = point_sample_bunny(&mut function_time_in_nano, &models, point_sampling_factor);

        let particles = pandemonium(&mut function_time_in_nano, &point_cloud);

        let kettle_done_in_time = kettle(&mut function_time_in_nano, target_temperature, max_time_steps);

        handle_workers(&mut function_time_in_nano, &mut workers, max_wait_time, worker_count);

        handle_union(&mut function_time_in_nano, &mut workers);

        manipulate_point_cloud(&mut function_time_in_nano, &mut point_cloud);

        update_screensaver(&mut function_time_in_nano, &mut screensaver);

        let geometry_machinery = build_geometry_machine(&mut function_time_in_nano, geometry_machine_thread_count, work_amount_per_gizmo, gizmos_per_machine);

        yell_at_workers(&mut function_time_in_nano, &mut workers);

        curvature_of_the_rainbow(&mut function_time_in_nano, kettle_done_in_time, &workers, &point_cloud, &particles, &geometry_machinery);

    }
    let loop_time_stop = Instant::now();
    total_time_in_nano.create_or_add_to_entry("Average Loop Time", (loop_time_stop - loop_time_start).as_nanos() / loop_count);
    total_time_in_nano.create_or_add_to_entry("Total Loop Time", (loop_time_stop - loop_time_start).as_nanos());
    total_time_in_nano.create_or_add_to_entry("Total Time", ((loop_time_stop - loop_time_start)).as_nanos() + total_time_in_nano.times.get("Preprocessing").unwrap().0);

    println!("FUNCTION TIMES:");
    function_time_in_nano.print_miliseconds();
    println!("");
    println!("TOTAL TIMES:");
    total_time_in_nano.print_miliseconds();
}
