use burn::{backend::Candle, nn::{conv::Conv2d, pool::AdaptiveAvgPool2d, Dropout}, prelude::*};

mod model;

type MyBackend = Candle<f32, i32>;
type MyAutodiffBackend = Autodiff<MyBackend>;




fn main() {
    println!("Hello, world!");
}
