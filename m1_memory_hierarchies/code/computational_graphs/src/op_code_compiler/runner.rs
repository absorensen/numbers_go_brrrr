use wgpu::ShaderModule;

use crate::shared::gpu_utilities::{create_shader_module, GPUHandles};

enum LinearOpCodes {
    Uniform,
    Bindings,
    FunctionDefinition,
    ThreadID,
    IndexCheck,
    OutputIndexCalculation,
    ResultInstation,
    MatrixMultiplicationLoop,
    AddBias,
    StoreResult,
    IndexCheckClose,
    FunctionClose,
}

struct Linear {
    op_codes: Vec<(LinearOpCodes, String)>,
}

// The represented shader is the same as found in shared::shaders::linear.wgsl
impl Linear {
    pub fn new() -> Self {
        let mut op_codes: Vec<(LinearOpCodes, String)> = vec![];
        let uniform: (LinearOpCodes, String) = (
            LinearOpCodes::Uniform,
            "        
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
            "
            .to_string(),
        );
        op_codes.push(uniform);

        let bindings: (LinearOpCodes, String) = (
            LinearOpCodes::Bindings,
            "        
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
            
            "
            .to_string(),
        );
        op_codes.push(bindings);

        let function_definition: (LinearOpCodes, String) = (
            LinearOpCodes::FunctionDefinition,
            "        
            const BLOCK_SIZE: u32 = 8u;
            @compute @workgroup_size(8, 8, 1) 
            fn main(
                @builtin(global_invocation_id) global_id: vec3<u32>,
                @builtin(workgroup_id) group_id: vec3<u32>, 
                @builtin(local_invocation_id) local_id: vec3<u32>
                ) {
            "
            .to_string(),
        );
        op_codes.push(function_definition);

        let thread_id: (LinearOpCodes, String) = (
            LinearOpCodes::ThreadID,
            "        
                let output_row_index: u32 = global_id.x;
                let output_column_index: u32 = global_id.y;
            "
            .to_string(),
        );
        op_codes.push(thread_id);

        let index_check: (LinearOpCodes, String) = (LinearOpCodes::IndexCheck,
            "        
                if (output_row_index < dimensions.output_row_count && output_column_index < dimensions.output_column_count) {
            ".to_string()
        );
        op_codes.push(index_check);

        let ouput_index_calculation: (LinearOpCodes, String) = (LinearOpCodes::OutputIndexCalculation,
            "        
                    let output_index: u32 = output_row_index * dimensions.output_column_count + output_column_index;
            ".to_string()
        );
        op_codes.push(ouput_index_calculation);

        let result_instantiation: (LinearOpCodes, String) = (
            LinearOpCodes::ResultInstation,
            "        
                    var result: f32 = 0.0;
            "
            .to_string(),
        );
        op_codes.push(result_instantiation);

        let matrix_multiplication_loop: (LinearOpCodes, String) = (LinearOpCodes::MatrixMultiplicationLoop,
            "        
                    for (var inner_dimension: u32 = 0u; inner_dimension < dimensions.input_column_count; inner_dimension += 1u) {
                            result += input[output_row_index * dimensions.input_column_count + inner_dimension] * weights[inner_dimension * dimensions.weights_column_count + output_column_index];
                    }
            ".to_string()
        );
        op_codes.push(matrix_multiplication_loop);

        let add_bias: (LinearOpCodes, String) = (
            LinearOpCodes::AddBias,
            "        
                    result = result + bias[output_index];
            "
            .to_string(),
        );
        op_codes.push(add_bias);

        let store_result: (LinearOpCodes, String) = (
            LinearOpCodes::StoreResult,
            "        
                    output[output_index] = result;
            "
            .to_string(),
        );
        op_codes.push(store_result);

        let index_check_close: (LinearOpCodes, String) = (
            LinearOpCodes::IndexCheckClose,
            "        
                }
            "
            .to_string(),
        );
        op_codes.push(index_check_close);

        let function_close: (LinearOpCodes, String) = (
            LinearOpCodes::FunctionClose,
            "        
            }
            "
            .to_string(),
        );
        op_codes.push(function_close);

        Self { op_codes }
    }
}

// Note that the handover between linear layer and relu could be done in a more generalized
// fashion by generating "handover"-variables during compilation. To keep things brief,
// this has been omitted, but it would look something like the Transfer and DeviceToDevice
// operators from the graph sections.
pub fn compile_linear_shader(gpu_handles: &GPUHandles, with_relu: bool) -> ShaderModule {
    let linear_op_codes: Linear = Linear::new();

    let mut string_builder: String = "".to_string();
    for op_code in linear_op_codes.op_codes {
        if with_relu {
            if let LinearOpCodes::StoreResult = op_code.0 {
                string_builder.push_str("        result = max(0.0, result);\n")
            }
        }

        string_builder.push_str(op_code.1.as_str());
    }

    create_shader_module(gpu_handles, string_builder.as_str())
}
