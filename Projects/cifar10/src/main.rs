use burn::{backend::{Autodiff, Candle}, prelude::*};

type MyBackend = Candle<f32, i64>;
type MyAutodiffBackend = Autodiff<MyBackend>;

pub struct Model<B: Backend> {
    linear1: 
}

fn main() {
    println!("Hello, world!");
}
