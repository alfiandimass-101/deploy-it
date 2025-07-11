use burn::{backend::{Autodiff, Candle}, nn::{Linear, LinearConfig, Relu}, prelude::*};

type MyBackend = Candle<f32, i64>;
type MyAutodiffBackend = Autodiff<MyBackend>;

#[derive(Debug, Module)]
pub struct Model<B: Backend>
{
    linear1: Linear<B>,
    linear2: Linear<B>,
    activation: Relu,
}

impl<B: Backend> Model<B> {
    pub fn new(input_s: usize, hidden_s: usize, output_s: usize) -> Self {
        let device = &Default::default();
        let linear1 = LinearConfig::new(input_s, hidden_s).init(device);
    }
}

fn main() {
    println!("Hello, world!");
}
