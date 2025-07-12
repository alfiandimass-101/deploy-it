use burn::{data::{dataloader::batcher::Batcher, dataset::vision::MnistItem}, prelude::*};

#[derive(Clone, Default)]
pub struct MnistBatcher {}

#[derive(Clone, Default)]
pub struct MnistBatch<B: Backend> {
    pub images: Tensor<B, 3>,
    pub targets: Tensor<B, 1, Int>
}

impl<B: Backend> Batcher<B, MnistItem, MnistBatch<B>> for MnistBatcher {
    fn batch(&self, items: Vec<MnistItem>, device: &<B as Backend>::Device) -> MnistBatch<B> {
        
    }
}