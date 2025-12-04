#![recursion_limit = "256"]

use burn::{backend::Wgpu, nn::Linear, prelude::*};

#[derive(Module, Debug)]
struct XorModel<B: Backend> {
    input: Linear<B>,
    hidden_layer: Linear<B>,
    hidden_layer_activation: ReLU<B>,
    output_layer: Linear<B>,
    output_layer_activation: Sigmoid<B>,
}

fn main() {
    // let device = Default::default();
}
