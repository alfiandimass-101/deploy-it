use burn::{
    nn::{
        Linear, LinearConfig, Relu, Sigmoid,
        loss::{BinaryCrossEntropyLoss, BinaryCrossEntropyLossConfig},
    },
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

impl XorModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> XorModel<B> {
        XorModel {
            input: LinearConfig::new(2, 4).init(device),
            hidden_layer: LinearConfig::new(4, 1).init(device),
            hidden_layer_activation: Relu::new(),
            output_layer: LinearConfig::new(1, 1).init(device),
            output_layer_activation: Sigmoid::new(),
            loss: BinaryCrossEntropyLossConfig::new().init(device),
        }
    }
}
