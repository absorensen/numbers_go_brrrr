struct TensorDimensions {
    tensor_a_row_count: u32,
    tensor_a_column_count: u32,
    tensor_b_row_count: u32,
    tensor_b_column_count: u32,
    output_row_count: u32,
    output_column_count: u32,
};

@group(0) @binding(0)
var<uniform> dimensions: TensorDimensions;

@group(0) @binding(1)
var<storage, read> tensor_a: array<f32>;

@group(0) @binding(2)
var<storage, read> tensor_b: array<f32>;

@group(0) @binding(3)
var<storage, read_write> output: array<f32>;

@compute @workgroup_size(32, 1, 1) 
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let output_row_index: u32 = global_id.x;
    let output_column_index: u32 = global_id.y;
    
    if (output_row_index < dimensions.output_row_count && output_column_index < dimensions.output_column_count) {
        let index: u32 = output_row_index * dimensions.output_column_count + output_column_index;
        output[index] = tensor_a[index] - tensor_b[index];
    }
}