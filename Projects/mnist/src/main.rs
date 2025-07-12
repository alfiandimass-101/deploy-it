use burn::{backend::{Autodiff, Candle}, prelude::*};

mod model;
mod data;
mod train;

use train::TrainingConfig;

type MyBackend = Candle<f32, i32>;
type MyAutodiffBackend = Autodiff<MyBackend>;

fn main() {
    let device = burn::backend::Candle::default();
    let artifact_dir = "/tmp/guide";
    crate::train::train::<MyAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
        device.clone(),
    );
}
