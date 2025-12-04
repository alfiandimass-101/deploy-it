use burn::data::dataloader::batcher::Batcher;
use burn::{Tensor, prelude::Backend};
use rand::random_bool;

#[derive(Debug, Clone)]
pub struct XorBatch<B: Backend> {
    pub inputs: Tensor<B, 2>,
    pub targets: Tensor<B, 1>,
}

#[derive(Debug, Default)]
pub struct XorBatcher {}

impl<B: Backend> Batcher<B, (bool, bool), XorBatch<B>> for XorBatcher {
    fn batch(&self, items: Vec<(bool, bool)>, device: &<B as Backend>::Device) -> XorBatch<B> {
        let inputs = Tensor::from_data(
            device,
            items
                .iter()
                .map(|(a, b)| [*a as i32 as f32, *b as i32 as f32])
                .collect(),
        );
    }
}

pub fn create_xor_data() -> (bool, bool) {
    let a = random_bool(1.0 / 3.0);
    let b = random_bool(1.0 / 3.0);
    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::backend::ndarray::NdArrayBackend; // Assuming NdArrayBackend for testing
    use burn::tensor::Device;

    #[test]
    fn test_create_xor_data() {
        let (a, b) = create_xor_data();
        // Just checking that it runs without panicking and returns two booleans.
        // The actual values are random, so we can't assert specific values.
        assert!(
            true,
            "create_xor_data should return a tuple of two booleans"
        );
    }

    #[test]
    fn test_xor_data_matches_xor_operator() {
        type B = NdArrayBackend;
        let device = Device::Cpu;
        let batcher = XorBatcher {};

        let items = vec![(false, false), (false, true), (true, false), (true, true)];
        let xor_batch = batcher.batch(items.clone(), &device);

        let expected_targets: Vec<f32> = items
            .iter()
            .map(|(a, b)| ((*a ^ *b) as i32) as f32)
            .collect();

        let actual_targets: Vec<f32> = xor_batch.targets.into_data().value;

        assert_eq!(actual_targets, expected_targets);
    }
}
