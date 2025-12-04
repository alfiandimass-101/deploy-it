#![recursion_limit = "256"]

use burn::{backend::Wgpu, nn::Linear, prelude::*};

#[derive(Module, Debug)]
struct XorModel<B: Backend> {
    input: Linear<B>,
    hidden_layer: Linear<B>,
}

fn main() {
    // let device = Default::default();
}
