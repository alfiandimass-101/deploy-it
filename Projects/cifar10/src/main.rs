use burn::{backend::{Autodiff, Candle}, nn::{conv::{Conv2d, Conv2dConfig}, pool::{MaxPool2d, MaxPool2dConfig}, Dropout, DropoutConfig, Linear, LinearConfig, Relu}, prelude::*};

mod model;

type MyBackend = Candle<f32, i64>;
type MyAutodiffBackend = Autodiff<MyBackend>;


fn main() {
    println!("Hello, world!");
}
