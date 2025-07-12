use burn::{backend::{Autodiff, Candle}, prelude::*};

mod model;

type MyBackend = Candle<f32, i32>;
type MyAutodiffBackend = Autodiff<MyBackend>;

fn main() {
    println!("Hello, world!");
}
