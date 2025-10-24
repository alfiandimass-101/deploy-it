use burn::{backend::{Autodiff, Wgpu}, prelude::*};

// Backend Register
type GlobalBackend = Wgpu<f32,i32,u32>;
type AutoDiffBackend = Autodiff<GlobalBackend>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut device = Device::default
    Ok(())
}