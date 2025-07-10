use burn::{backend::{Autodiff, Candle}, nn::{conv::Conv2d, pool::MaxPool2d, Relu}, prelude::*};

// Backend for MNIST model (using cpu with candle)
// f32 for calculation and i64 for the dataset label
type MyBackend = Autodiff<Candle<f32, i64>>;

#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    pool: MaxPool2d,
    relu: Relu,
    linear1: Linear<B>,
    linear2: Linear<B>,
}

fn main() {
    println!("Hello, world!");
}
