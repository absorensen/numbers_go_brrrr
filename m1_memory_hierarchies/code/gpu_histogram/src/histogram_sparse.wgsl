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

var<private> local_entries: array<u32, ELEMENTS_PER_THREAD>;
var<private> local_counts: array<u32, ELEMENTS_PER_THREAD>;

@compute @workgroup_size(32, 1, 1) 
fn histogram(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(workgroup_id) group_id: vec3<u32>, 
    @builtin(local_invocation_id) local_id: vec3<u32>
    ) {
        var global_index: u32 = group_id.x * ELEMENTS_PER_THREAD * 32u + local_id.x;
        var unoccupied_index: u32 = 0u;
        if global_index < dimensions.element_count {
            for(var elements_fetched: u32 = 0u; elements_fetched < ELEMENTS_PER_THREAD; elements_fetched += 1u) {
                let entry: u32 = u32(floor(input[global_index]));
                var sparse_index: u32 = 0u;
                while (sparse_index < unoccupied_index ) {
                    if (local_entries[sparse_index] == entry) {
                        local_counts[sparse_index] += 1u;
                        break;
                    }
                    sparse_index += 1u;
                }

                if (sparse_index == unoccupied_index) {
                    unoccupied_index += 1u;
                    local_counts[sparse_index] = 1u;
                    local_entries[sparse_index] = entry;
                }

                global_index += 32u;
                if (dimensions.element_count <= global_index) {
                    break;
                }
            }
        }

        for(var local_index: u32 = 0u; local_index < ELEMENTS_PER_THREAD; local_index += 1u) {
            if (0u < local_counts[local_index]) {
                let entry: u32 = local_entries[local_index];
                atomicAdd(&shared_histogram[entry], local_counts[local_index]);
            }
        }

        workgroupBarrier();

        var local_index: u32 = local_id.x;
        while (local_index < BIN_COUNT) {
            if (shared_histogram[local_index] != 0u) {
                atomicAdd(&output[local_index], shared_histogram[local_index]);
            }
            local_index += 32u;
        }
}