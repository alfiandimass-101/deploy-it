use burn::{backend::{ndarray::NdArrayDevice, Autodiff, NdArray}, optim::AdamConfig, prelude::*};

mod model;
mod data;
mod train;

use train::TrainingConfig;
use model::{Model, ModelConfig};

type MyBackend = NdArray<f32>;
type MyAutodiffBackend = Autodiff<MyBackend>;

fn main() {
   neof let device = NdArrayDevice::default();
    let artifact_dir = "../model";
    crate::train::train::<MyAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
        device.clone(),
    );
}
