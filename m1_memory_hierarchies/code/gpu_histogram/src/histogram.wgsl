struct Uniform {
    element_count: u32,
    not_used: u32,
    not_used: u32,
    not_used: u32,
};

@group(0) @binding(0)
var<uniform> dimensions: Uniform;

@group(0) @binding(1)
var<storage, read> input: array<f32>;

// Bind a read/write array
@group(0) @binding(2)
var<storage, read_write> output: array<u32>;

@compute @workgroup_size(32, 1, 1) 
fn histogram(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    ) {
    let thread_id: u32 = global_id.x;
    
    if (thread_id < dimensions.element_count) {
        let index: u32 = u32(floor(input[thread_id]));
        output[index] += 1u;        
    }
}