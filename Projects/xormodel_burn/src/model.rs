use burn::{
    nn::{Linear, Relu, Sigmoid, loss::BinaryCrossEntropyLoss},
    prelude::*,
};

#[derive(Module, Debug)]
struct XorModel<B: Backend> {
    input: Linear<B>,
    hidden_layer: Linear<B>,
    hidden_layer_activation: Relu,
    output_layer: Linear<B>,
    output_layer_activation: Sigmoid,
    loss: BinaryCrossEntropyLoss<B>,
}

#[derive(Config, Debug)]
struct XorModelConfig {
    hidden_size: usize,
}
