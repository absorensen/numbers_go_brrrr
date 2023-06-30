struct TensorDimensions {
    data_row_count: u32,
    data_column_count: u32,
};

@group(0) @binding(0)
var<uniform> dimensions: TensorDimensions;

@group(0) @binding(1)
var<storage, read_write> data: array<f32>;

@compute @workgroup_size(32, 1, 1) 
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let data_row_index: u32 = global_id.x;
    let data_column_index: u32 = global_id.y;
    
    if (data_row_index < dimensions.data_row_count && data_column_index < dimensions.data_column_count) {
        let index: u32 = data_row_index * dimensions.data_column_count + data_column_index;
        data[index] = max(0.0, data[index]);
    }
}