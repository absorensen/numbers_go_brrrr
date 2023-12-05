struct TensorDimensions {
    input_row_count: u32,
    input_column_count: u32,
    weights_row_count: u32,
    weights_column_count: u32,
    bias_row_count: u32,
    bias_column_count: u32,
    output_row_count: u32,
    output_column_count: u32,
};

@group(0) @binding(0)
var<uniform> dimensions: TensorDimensions;

@group(0) @binding(1)
var<storage, read> input: array<f32>;

@group(0) @binding(2)
var<storage, read> weights: array<f32>;

@group(0) @binding(3)
var<storage, read> bias: array<f32>;

@group(0) @binding(4)
var<storage, read_write> output: array<f32>;

const BLOCK_SIZE: u32 = 8u;
@compute @workgroup_size(8, 8, 1) 
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(workgroup_id) group_id: vec3<u32>, 
    @builtin(local_invocation_id) local_id: vec3<u32>
    ) {
    let output_row_index: u32 = global_id.x;
    let output_column_index: u32 = global_id.y;

    if (output_row_index < dimensions.output_row_count && output_column_index < dimensions.output_column_count) {
        let output_index: u32 = output_row_index * dimensions.output_column_count + output_column_index;
        var result: f32 = 0.0;
        for (var inner_dimension: u32 = 0u; inner_dimension < dimensions.input_column_count; inner_dimension += 1u) {
            result += input[output_row_index * dimensions.input_column_count + inner_dimension] * weights[inner_dimension * dimensions.weights_column_count + output_column_index];
        }
        
        output[output_index] = result + bias[output_index];
    }
}

@compute @workgroup_size(8, 8, 1) 
fn main_with_relu(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(workgroup_id) group_id: vec3<u32>, 
    @builtin(local_invocation_id) local_id: vec3<u32>
    ) {
    let output_row_index: u32 = global_id.x;
    let output_column_index: u32 = global_id.y;

    if (output_row_index < dimensions.output_row_count && output_column_index < dimensions.output_column_count) {
        let output_index: u32 = output_row_index * dimensions.output_column_count + output_column_index;

        var result: f32 = 0.0;
        for (var inner_dimension: u32 = 0u; inner_dimension < dimensions.input_column_count; inner_dimension += 1u) {
            result += input[output_row_index * dimensions.input_column_count + inner_dimension] * weights[inner_dimension * dimensions.weights_column_count + output_column_index];
        }

        output[output_index] = max(0.0, result + bias[output_index]);
    }
}