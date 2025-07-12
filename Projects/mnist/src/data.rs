use burn::{data::{dataloader::batcher::Batcher, dataset::vision::MnistItem}, prelude::*, tensor::ops::{FloatElem, IntElem}};

#[derive(Clone, Default)]
pub struct MnistBatcher {}

#[derive(Clone, Default)]
pub struct MnistBatch<B: Backend> {
    pub images: Tensor<B, 3>,
    pub targets: Tensor<B, 1, Int>
}

impl<B: Backend> Batcher<B, MnistItem, MnistBatch<B>> for MnistBatcher {
    fn batch(&self, items: Vec<MnistItem>, device: &<B as Backend>::Device) -> MnistBatch<B> {
        let images = items.iter()
            .map(|item| TensorData::from(item.image).convert::<FloatElem<B>>())
            .map(|data| Tensor::from_data(data, device))
            .map(|tensor| tensor.reshape([1,28,28]))
            .map(|tensor| ((tensor/255)- 0.1307) / 0.3081)
            .collect();
        let targets = items.iter()
            .map(|item| Tensor::from_data([(item.label as i64).elem::<IntElem<B>>()], device))
    }
}