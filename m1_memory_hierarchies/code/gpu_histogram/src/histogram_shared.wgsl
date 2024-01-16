// We would have to hardcode this line if we didn't use the bin_count specialization
// when compiling the shader
//const BIN_COUNT: u32 = 5u;

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
var<storage, read> input: array<f32>;

// Bind a read/write array
@group(0) @binding(2)
var<storage, read_write> output: array<atomic<u32>, BIN_COUNT>;

var<workgroup> shared_histogram: array<atomic<u32>, BIN_COUNT>;

@compute @workgroup_size(32, 1, 1) 
fn histogram(
    // For this example we only need access to the global
    // thread ID.
    @builtin(global_invocation_id) global_id: vec3<u32>,
    // But you can also gain other more localized ID's
    //@builtin(workgroup_id) group_id: vec3<u32>, 
    @builtin(local_invocation_id) local_id: vec3<u32>
    ) {
    // Make sure we are inside the valid range of the
    // arrays, if not, do nothing.
    let thread_id: u32 = global_id.x;
    if (thread_id < dimensions.element_count) {
        let index: u32 = u32(floor(input[thread_id]));
        atomicAdd(&shared_histogram[index], 1u);
    }

    workgroupBarrier();

    var local_index: u32 = local_id.x;
    while (local_index < BIN_COUNT) {
        atomicAdd(&output[local_index], shared_histogram[local_index]);
        local_index += 32u;
    }
}