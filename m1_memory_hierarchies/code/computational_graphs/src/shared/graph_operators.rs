use super::tensor2d::Tensor2D;

#[derive(Clone, Debug)]
pub enum GraphOperator {
    Empty,
    HostToDevice { input: Tensor2D },
    DeviceToHost,
    LinearLayer { weights: Tensor2D, bias: Tensor2D },
    ReLU,
    Softmax,
    LinearReLUFused { weights: Tensor2D, bias: Tensor2D },
    LinearReLUSoftmaxFused { weights: Tensor2D, bias: Tensor2D },
}
