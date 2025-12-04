use burn::data::dataloader::batcher::Batcher;
use burn::{Tensor, prelude::Backend, tensor::TensorData};
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
        let inputs = items
            .iter()
            .flat_map(|(a, b)| [*a as i32 as f32, *b as i32 as f32])
            .collect::<Vec<_>>();

        let targets = items
            .iter()
            .map(|(a, b)| (*a ^ *b) as i32 as f32)
            .collect::<Vec<_>>();

        let shape = [items.len(), 2];
        let inputs = Tensor::from_data(TensorData::new(inputs, shape), device);

        let targets = Tensor::from_data(TensorData::new(targets, [items.len()]), device);

        XorBatch { inputs, targets }
    }
}

pub fn create_xor_data() -> (bool, bool) {
    let a = random_bool(1.0 / 3.0);
    let b = random_bool(1.0 / 3.0);
    (a, b)
}

#[test]
fn test_xor_logic() {
    let test_cases = vec![
        (false, false, false),
        (false, true, true),
        (true, false, true),
        (true, true, false),
    ];

    for (a, b, expected) in test_cases {
        let result = a ^ b;
        assert_eq!(
            result, expected,
            "XOR of ({}, {}) should be {}",
            a, b, expected
        );
    }
}

#[test]
fn test_batcher() {
    use burn_ndarray::NdArray;
    let batcher = XorBatcher::default();
    let items = vec![(true, false), (false, true)];
    let device = Default::default();
    let batch: XorBatch<NdArray> = batcher.batch(items, &device);
    assert_eq!(batch.inputs.shape().dims, [2, 2]);
    assert_eq!(batch.targets.shape().dims, [2]);
}
