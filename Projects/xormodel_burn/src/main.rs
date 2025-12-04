#![recursion_limit = "256"]

use burn::backend::Wgpu;
use burn::prelude::*;

mod model;

type InferenceBackend = Wgpu;

fn main() {
    let device = Default::default();

    let model = model::XorModelConfig::new(4).init::<InferenceBackend>(&device);
    println!("Model: {:#?}", model);
}
