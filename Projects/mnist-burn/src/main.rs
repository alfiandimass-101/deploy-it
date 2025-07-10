use burn::{backend::{Autodiff, Candle}, nn::{conv::{Conv2d, Conv2dConfig}, pool::{MaxPool2d, MaxPool2dConfig}, Linear, LinearConfig, Relu}, prelude::*};

// Backend for MNIST model (using cpu with candle)
// f32 for calculation and i64 for the dataset label
type MyBackend = Autodiff<Candle<f32, i64>>;

// Model for MNIST (as the brain)
#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    pool: MaxPool2d,
    relu: Relu,
    linear1: Linear<B>, // input
    linear2: Linear<B>, //output
}

impl<B: Backend> Model<B> {
    // create new model
    pub fn new(num_class: usize) -> Self {
        // configuration of the layers.
        let device = &Default::default();
        let conv1 = Conv2dConfig::new([1, 8], [3,3]).init(device);
        let conv2 = Conv2dConfig::new([8, 16], [3,3]).init(device);
        let pool = MaxPool2dConfig::new([2,2]).with_strides([2,2]).init();
        let relu = Relu::new();
        let linear1 = LinearConfig::new(16*5*5, 128).init(device);
        let linear2 = LinearConfig::new(128, num_class).init(device);
        Self { conv1, conv2, pool, relu, linear1, linear2 } 
    }
}

fn main() {
    println!("Hello, world!");
}
