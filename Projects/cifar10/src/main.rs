use burn::{backend::{Autodiff, Candle}, nn::{conv::{Conv2d, Conv2dConfig}, pool::{MaxPool2d, MaxPool2dConfig}, Dropout, DropoutConfig, Linear, LinearConfig, Relu}, prelude::*};

type MyBackend = Candle<f32, i64>;
type MyAutodiffBackend = Autodiff<MyBackend>;

#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    pool: MaxPool2d,
    dropout: Dropout,
    linear1: Linear<B>,
    linear2: Linear<B>,
    activation: Relu,
}

impl<B: Backend> Model<B> {
    pub fn new(num_classes: usize) -> Self {
        let device = &Default::default();
        // [B, 3, 32, 32] -> [B, 32, 30, 30]
        let conv1 = Conv2dConfig::new([3, 32], [3,3]).init(device);
        // [B, 32, 30, 30] -> [B, 32, 15, 15]
        let pool = MaxPool2dConfig::new([2,2]).init();
        // [B, 32, 15, 15] -> [B, 64, 13, 13]
        let conv2 = Conv2dConfig::new([32, 64], [3,3]).init(device);
        // [B, 64, 13, 13] -> [B, 64, 6, 6]
        let dropout = DropoutConfig::new(0.5).init();
        
        let linear1 = LinearConfig::new(16*6*6, 512).init(device);
        let linear2 = LinearConfig::new(512, num_classes).init(device);
    }
}

fn main() {
    println!("Hello, world!");
}
