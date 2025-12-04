#![recursion_limit = "256"]

use burn::{
    backend::Wgpu,
    nn::{Linear, Relu, Sigmoid},
    prelude::*,
};

#[derive(Module, Debug)]
struct XorModel<B: Backend> {
    input: Linear<B>,
    hidden_layer: Linear<B>,
    hidden_layer_activation: Relu,
    output_layer: Linear<B>,
    output_layer_activation: Sigmoid,
}

fn main() {
    // let device = Default::default();
}
