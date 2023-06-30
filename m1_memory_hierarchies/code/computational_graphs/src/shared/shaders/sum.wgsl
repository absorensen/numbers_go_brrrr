const BLOCK_SIZE: u32 = 32u;

struct SumUniform {
    element_count: u32,
    block_count: u32,
};

@group(0) @binding(0)
var<uniform> sum_uniform: SumUniform;

@group(0) @binding(1)
var<storage, read> data: array<f32>;

// Should have size 1. Output sum will be written to index 0.
@group(0) @binding(2)
var<storage, read_write> output: array<f32>;

var<workgroup> shared_data: array<f32, BLOCK_SIZE>;


// In general it needs to be verified how we handle odd sizes
// We could make this shader faster by demanding that all
// Input arrays were N*32 in size. The remaining values
// could just be 0.0. This does however put more responsibility
// on to the user or the next programmer who has to know
// what we expect. Another option could be to determine 
// whether to use the N*32 shader or the more robust shader
// cpu-side and launch the correct one.
@compute @workgroup_size(32, 1, 1) 
fn global_phase(
    @builtin(workgroup_id) group_id: vec3<u32>, 
    @builtin(local_invocation_id) local_id: vec3<u32>,
    ) {

    // // Calculate starting indices
    // var tid: u32 = local_id.x;
    // var i: u32 = group_id.x * (BLOCK_SIZE / 2u) + local_id.x;

    // // Load global variables into shared memory
    // shared_data[tid] = data[i];

    // if ((i + BLOCK_SIZE) < sum_uniform.element_count) {
    //     shared_data[tid] += data[i + BLOCK_SIZE];
    // }
    // workgroupBarrier();
     
    // // Compute in shared memory
    // // Should perhaps be a function
    // for(var s: u32 = BLOCK_SIZE / 2u; s > 0u; s >>= 1u) { 
    //     if(tid < s) {
    //         shared_data[tid] += shared_data[tid + s];
    //     }

    //     workgroupBarrier();
    // }


    let tid: u32 = local_id.x;
    var index: u32 = group_id.x * BLOCK_SIZE + local_id.x;

    // Needs to be ttested with an odd number of elements
    if (index < sum_uniform.element_count) {
        // Load global variables into shared memory
        shared_data[tid] = data[index];
        index += sum_uniform.block_count * BLOCK_SIZE;
        if (index < sum_uniform.element_count) {
            shared_data[tid] += data[index];
        }

        // There is a more effective way to do the reduction at this stage
        // but it needs a case for when all 32 shared memory slots have data
        // and one for when less than 32 slots have memory

        // Put subresults into global array
        if (tid == 0u) {
            let element_count: u32 = BLOCK_SIZE;
            // if ((sum_uniform.element_count - group_id.x * BLOCK_SIZE) < BLOCK_SIZE) {
            //     element_count = sum_uniform.element_count - index;
            // }

            var sum: f32 = 0.0;
            for(var index: u32 = 0u; index < element_count; index += 1u){
                sum += shared_data[index];
            }

            output[group_id.x] = sum;
        }
    }
}

// In general it needs to be verified how we handle odd sizes
// This function should only ever be launched for a single workgroup
@compute @workgroup_size(32, 1, 1) 
fn workgroup_phase(
    @builtin(workgroup_id) group_id: vec3<u32>, 
    @builtin(local_invocation_id) local_id: vec3<u32>,
    ) {
    var tid: u32 = local_id.x;
    if (group_id.x == 0u) {
        // In this first section we can use all 32 threads
        var elements_left: u32 = sum_uniform.block_count;
        var i: u32 = tid;
        var sum_value: f32 = 0.0;
        // How do we handle the odd case?
        while (BLOCK_SIZE < elements_left) {
            sum_value += data[i];
            elements_left -= BLOCK_SIZE;
            i += BLOCK_SIZE;
        }
        if (0u < elements_left) {
            if(tid < elements_left) {
                sum_value += data[i];
            }
        }

        shared_data[tid] = sum_value;
        workgroupBarrier();


        // In this section we can use at most 16 threads, but it will decrease with every loop iteration
        // How does s relate to elements_left?
        // How do we handle odd sizes?
        // See if this can be made to work later. It does not currently work with elements_left != 2^n
        // for(var s: u32 = elements_left >> 1u; s > 0u; s >>= 1u) { 
        //     if(tid < s) {
        //         shared_data[tid] += shared_data[tid + s];
        //     }

        //     workgroupBarrier();
        // }

                // There is a more effective way to do the reduction at this stage
        // but it needs a case for when all 32 shared memory slots have data
        // and one for when less than 32 slots have memory

        if (tid == 0u) {
            //Handle the case where elements_left was odd
            // if (elements_left != ((elements_left >> 1u) * 2u)) {
            //     shared_data[tid] += shared_data[elements_left - 1u];
            // }

            var sum: f32 = 0.0;
            var index: u32 = 0u;
            while (index < elements_left) {
                sum += shared_data[index];
                index++;
            }
            // var sum: f32 = 0.0;
            // for(var index: u32 = 0u; index < elements_left; index += 1u){
            //     sum += data[index];
            // }
            output[0] = sum;
            // output[0] = sum;

        }
    }
}

// In general it needs to be verified how we handle odd sizes
// This function should only ever be launched for a single workgroup
@compute @workgroup_size(32, 1, 1) 
fn single_pass_sum(
    @builtin(workgroup_id) group_id: vec3<u32>, 
    @builtin(local_invocation_id) local_id: vec3<u32>,
    ) {
    let tid: u32 = local_id.x;
    // In this first section we can use all 32 threads
    var elements_left: u32 = sum_uniform.element_count;
    var i: u32 = tid;
    var sum_value: f32 = 0.0;
    // How do we handle the odd case?
    while (BLOCK_SIZE < elements_left) {
        sum_value += data[i];
        elements_left -= BLOCK_SIZE;
        i += BLOCK_SIZE;
    }
    if(tid < elements_left) {
        sum_value += data[i];
    }

    shared_data[tid] = sum_value;
    workgroupBarrier();


    // In this section we can use at most 16 threads, but it will decrease with every loop iteration
    // How does s relate to elements_left?
    // How do we handle odd sizes?
    // See if this can be made to work later. It does not currently work with elements_left != 2^n
    // for(var s: u32 = elements_left >> 1u; s > 0u; s >>= 1u) { 
    //     if(tid < s) {
    //         shared_data[tid] += shared_data[tid + s];
    //     }

    //     workgroupBarrier();
    // }

            // There is a more effective way to do the reduction at this stage
    // but it needs a case for when all 32 shared memory slots have data
    // and one for when less than 32 slots have memory

    //Handle the case where elements_left was odd
    // In that case we know there was launched at least 32 threads
    // Each of them put either a live value or a zero into shared memory
    // This should be fine to look at later (but move it outside of tid==0u)
    // if (elements_left != ((elements_left >> 1u) * 2u)) {
    //     shared_data[tid] += shared_data[elements_left - 1u];
    // }

    if (tid == 0u) {
        var sum_value: f32 = 0.0;
        var index: u32 = 0u;
        while (index < BLOCK_SIZE) {
            sum_value += shared_data[index];
            index++;
        }
        // var sum: f32 = 0.0;
        // for(var index: u32 = 0u; index < elements_left; index += 1u){
        //     sum += data[index];
        // }
        output[0] = sum_value;
        // output[0] = sum;

    }
}