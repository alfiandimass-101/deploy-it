use burn::prelude::*;

type MyBackend = Candle<f32, i64>;
type MyAutodiffBackend = Autodiff<MyBackend>;


fn main() {
    println!("Hello, world!");
}
