const BLOCK_SIZE: u32 = 32u;

struct SoftmaxUniform {
    element_count: u32,
};

@group(0) @binding(0)
var<uniform> softmax_uniform: SoftmaxUniform;

@group(0) @binding(1)
var<storage, read> input: array<f32>;

@group(0) @binding(2)
var<storage, read_write> global_max: array<f32>;

@group(0) @binding(3)
var<storage, read_write> global_offset: array<f32>;

@group(0) @binding(4)
var<storage, read_write> output: array<f32>;

var<workgroup> shared_data: array<f32, BLOCK_SIZE>;

@compute @workgroup_size(32, 1, 1) 
fn single_pass_max(
    @builtin(workgroup_id) group_id: vec3<u32>, 
    @builtin(local_invocation_id) local_id: vec3<u32>,
    ) {
    let tid: u32 = local_id.x;
    // In this first section we can use all 32 threads
    var elements_left: u32 = softmax_uniform.element_count;
    var i: u32 = tid;
    var max_value: f32 = -3.00282346638528859812e+37f;
    // How do we handle the odd case?
    while (BLOCK_SIZE < elements_left) {
        max_value = max(max_value, input[i]);
        elements_left -= BLOCK_SIZE;
        i += BLOCK_SIZE;
    }
    if(tid < elements_left) {
        max_value = max(max_value, input[i]);
    }

    shared_data[tid] = max_value;
    workgroupBarrier();


    if (tid == 0u) {
        var max_value: f32 = shared_data[0];
        var index: u32 = 1u;
        while (index < BLOCK_SIZE) {
            max_value = max(max_value, shared_data[index]);
            index++;
        }
        global_max[0] = max_value;
    }
}

@compute @workgroup_size(32, 1, 1) 
fn single_pass_sum(
    @builtin(workgroup_id) group_id: vec3<u32>, 
    @builtin(local_invocation_id) local_id: vec3<u32>,
    ) {
    let tid: u32 = local_id.x;
    // In this first section we can use all 32 threads
    var elements_left: u32 = softmax_uniform.element_count;
    var index: u32 = tid;
    var sum_value: f32 = 0.0;
    let max_value: f32 = global_max[0];
    // How do we handle the odd case?
    while (BLOCK_SIZE < elements_left) {
        sum_value += exp(input[index] - max_value);
        elements_left -= BLOCK_SIZE;
        index += BLOCK_SIZE;
    }
    if(tid < elements_left) {
        sum_value += exp(input[index] - max_value);
    }

    shared_data[tid] = sum_value;
    workgroupBarrier();


    if (tid == 0u) {
        var sum_value: f32 = 0.0;
        var index: u32 = 0u;
        while (index < BLOCK_SIZE) {
            sum_value += shared_data[index];
            index++;
        }
        global_offset[0] = max_value + log(sum_value);
    }
}

@compute @workgroup_size(32, 1, 1) 
fn map(
    @builtin(workgroup_id) group_id: vec3<u32>, 
    @builtin(local_invocation_id) local_id: vec3<u32>,
) {
    let tid: u32 = local_id.x;
    let index: u32 = group_id.x * BLOCK_SIZE + local_id.x;

    if (index < softmax_uniform.element_count) {
        output[index] = exp(input[index] - global_offset[0]);
    }
}