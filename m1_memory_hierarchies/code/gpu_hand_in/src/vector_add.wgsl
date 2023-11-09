// This doesn't actually need to be a
// struct. We could just have u32,
// but if we were using two- or more
// dimensions, having more dimensions
// would greatly cut down on the 
// amount of binding slots used.
// There is a limitation on those.

struct Uniform {
    element_count: u32,
    not_used: u32,
    not_used: u32,
    not_used: u32,
};

// We can have different bind groups
// which each have their own set of bindings.
// var<uniform> means it is a read-only
// set of values which all threads can safely
// load in its entirety and it won't be updated.
@group(0) @binding(0)
var<uniform> dimensions: Uniform;

// Bind a read only array of 32-bit floats
@group(0) @binding(1)
var<storage, read> input_a: array<f32>;

@group(0) @binding(2)
var<storage, read> input_b: array<f32>;

// Bind a read/write array
@group(0) @binding(3)
var<storage, read_write> output: array<f32>;

// With @compute we define that this is a compute shader.
// With @workgroup_size we define the size of the warp. 
// If we had a lot of register pressure we might
// define it as some lower 2^n number instead,
// or if we were doing something two-dimensional,
// it might for example make sense to use @workgroup_size(8, 8, 1)
// or @workgroup_size(16, 2, 1)
@compute @workgroup_size(32, 1, 1) 
fn vector_add(
    // For this example we only need access to the global
    // thread ID.
    @builtin(global_invocation_id) global_id: vec3<u32>,
    // But you can also gain other more localized ID's
    //@builtin(workgroup_id) group_id: vec3<u32>, 
    //@builtin(local_invocation_id) local_id: vec3<u32>
    ) {
    // Get the global thread ID. Since we are
    // only working in one dimension, we only
    // need the ID in one dimension.
    let thread_id: u32 = global_id.x;
    
    // Make sure we are inside the valid range of the
    // arrays, if not, do nothing.
    if (thread_id < dimensions.element_count) {
        output[thread_id] = input_a[thread_id] + input_b[thread_id];        
    }
}