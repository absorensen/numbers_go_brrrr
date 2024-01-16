mod utility;
mod gpu_vector_f32;
mod gpu_vector_u32;
mod histogram;
use crate::histogram::histogram;

use utility::{self_test, GPUHandles, initialize_gpu};

fn main() {
    // Initialize the env_logger to get usueful messages from wgpu.
    env_logger::init();

    // Is there a compatible GPU on the system?
    // Use pollster::block_on to block on async functions.
    // Think of it like this - this is a function which
    // uses the GPU. With block_on() we are insisting
    // on waiting until all the interaction with the GPU
    // and the tasks set in motion on the GPU are finished.
    if !pollster::block_on(self_test()) {
        panic!("Was unable to confirm that your system is compatible with this sample!");
    }

    // Keep track of the handles to central stuff like device and queue.
    let handles: GPUHandles = pollster::block_on(initialize_gpu()).expect("Was unsuccesful in creating GPU Handles");

    assert!(histogram(&handles));
}

