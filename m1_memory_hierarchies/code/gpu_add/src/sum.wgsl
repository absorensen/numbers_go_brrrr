struct TensorDimensions {
    element_count: u32,
};

@group(0) @binding(0)
var<uniform> dimensions: TensorDimensions;

@group(0) @binding(1)
var<storage, read> input_a: array<f32>;

@group(0) @binding(2)
var<storage, read> input_b: array<f32>;

@group(0) @binding(3)
var<storage, read_write> output: array<f32>;

const BLOCK_SIZE: u32 = 32u;
@compute @workgroup_size(32, 1, 1) 
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(workgroup_id) group_id: vec3<u32>, 
    @builtin(local_invocation_id) local_id: vec3<u32>
    ) {
    let thread_id: u32 = global_id.x;
    
    if (thread_id < dimensions.element_count) {
        output[thread_id] = input_a[thread_id] + input_b[thread_id];        
    }
}