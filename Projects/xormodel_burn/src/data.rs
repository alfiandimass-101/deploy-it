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
        let inputs = items
            .iter()
            .map(|(a, b)| [*a as i32 as f32, *b as i32 as f32])
            .collect::<Vec<_>>();

        let targets = items
            .iter()
            .map(|(a, b)| (*a ^ *b) as i32 as f32)
            .collect::<Vec<_>>();

        let inputs = Tensor::from_data(inputs.as_slice(), device);

        let targets = Tensor::from_data(targets.as_slice(), device);

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
