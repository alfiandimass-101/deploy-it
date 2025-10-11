use burn::{backend::{Autodiff, Wgpu}, prelude::*};

type ProjectBackend = Wgpu<f32, i32, u32>;
type AutoDiffBackend = Autodiff<ProjectBackend>;

fn main() -> anyhow::Result<()> {
    let device = Device::<ProjectBackend>::default();
    println!("{}", ProjectBackend::name(&device));
    Ok(())
}
