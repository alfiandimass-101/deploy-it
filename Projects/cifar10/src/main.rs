use burn::{backend::{Autodiff, Candle}};

mod model;

type MyBackend = Candle<f32, i64>;
type MyAutodiffBackend = Autodiff<MyBackend>;


fn main() {
    println!("Hello, world!");
}
