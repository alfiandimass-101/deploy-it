use burn::{backend::{Autodiff, Candle}, nn::{Linear, Relu}, prelude::*};

type MyBackend = Candle<f32, i64>;
type MyAutodiffBackend = Autodiff<MyBackend>;

#[derive(Debug, Module)]
pub struct Model<B>
where B: Backend
{
    linear1: Linear<B>,
    linear2: Linear<B>,
    activation: Relu,
}

fn main() {
    println!("Hello, world!");
}
