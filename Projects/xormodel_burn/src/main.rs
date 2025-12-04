#![recursion_limit = "256"]

use burn::{backend::Wgpu, prelude::*};

type Backend = Wgpu<f32, i32, u32>;

fn main() {
    let device = Default::default();

    let tensor1 = Tensor::<Backend, 2, Int>::from_data([[2, 3], [4, 5]], &device);
    let tensor2 = Tensor::<Backend, 2, Int>::ones_like(&tensor1);

    println!("tensor1: {}", tensor1);
    println!("tensor2: {}", tensor2);
    println!("tensor1 + tensor2: {}", tensor1 + tensor2);
}
