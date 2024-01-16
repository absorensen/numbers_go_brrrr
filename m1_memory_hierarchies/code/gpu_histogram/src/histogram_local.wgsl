// We would have to hardcode this line if we didn't use the bin_count specialization
// when compiling the shader
//const BIN_COUNT: u32 = 5u;
//const ELEMENTS_PER_THREAD: u32 = 256u;

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

@group(0) @binding(2)
var<storage, read_write> output: array<atomic<u32>, BIN_COUNT>;

var<workgroup> shared_histogram: array<atomic<u32>, BIN_COUNT>;

var<private> local_histogram: array<u32, BIN_COUNT>;

@compute @workgroup_size(32, 1, 1) 
fn histogram(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(workgroup_id) group_id: vec3<u32>, 
    @builtin(local_invocation_id) local_id: vec3<u32>
    ) {
        var index: u32 = group_id.x * ELEMENTS_PER_THREAD * 32u + local_id.x;
        if index < dimensions.element_count {
            for(var elements_fetched: u32 = 0u; elements_fetched < ELEMENTS_PER_THREAD; elements_fetched += 1u) {
                local_histogram[u32(floor(input[index]))] += 1u;
                index += 32u;
                if (dimensions.element_count <= index) {
                    break;
                }
            }
        }

        workgroupBarrier();

        for(var local_index: u32 = 0u; local_index < BIN_COUNT; local_index += 1u) {
            atomicAdd(&shared_histogram[local_index], local_histogram[local_index]);
        }

        workgroupBarrier();

        var local_index: u32 = local_id.x;
        while (local_index < BIN_COUNT) {
            atomicAdd(&output[local_index], shared_histogram[local_index]);
            local_index += 32u;
        }
}