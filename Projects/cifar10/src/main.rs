use burn::{backend::{Autodiff, Candle}, nn::{conv::Conv2d, pool::MaxPool2d, Dropout, Linear, Relu}, prelude::*};

type MyBackend = Candle<f32, i64>;
type MyAutodiffBackend = Autodiff<MyBackend>;

pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    pool: MaxPool2d,
    dropout: Dropout,
    linear1: Linear<B>,
    linear2: Linear<B>,
    activation: Relu,
}



fn main() {
    println!("Hello, world!");
}
