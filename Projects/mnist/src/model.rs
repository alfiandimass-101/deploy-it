// 1. Import the new modules you need: MaxPool2d and MaxPool2dConfig
use burn::{
    nn::{
        conv::{Conv2d, Conv2dConfig},
        loss::CrossEntropyLossConfig,
        pool::{MaxPool2d, MaxPool2dConfig}, // MODIFIED: Changed imports
        Dropout, DropoutConfig, Linear, LinearConfig, Relu,
    },
    prelude::*,
    tensor::backend::AutodiffBackend,
    train::{ClassificationOutput, TrainOutput, TrainStep, ValidStep},
};

use crate::data::MnistBatch;

#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    // 2. Change the type of the 'pool' field in the model struct
    pool: MaxPool2d, // MODIFIED: Was AdaptiveAvgPool2d
    dropout: Dropout,
    linear1: Linear<B>,
    linear2: Linear<B>,
    activation: Relu,
}

#[derive(Debug, Config)]
pub struct ModelConfig {
    num_classes: usize,
    hidden_size: usize,
    #[config(default = "0.5")]
    drop_prob: f64,
}

impl ModelConfig {
    pub fn init<B: Backend>(&self) -> Model<B> {
        let device = &Default::default();
        let conv1 = Conv2dConfig::new([1, 8], [3, 3]).init(device);
        let conv2 = Conv2dConfig::new([8, 16], [3, 3]).init(device);

        // 3. Initialize MaxPool2d instead of AdaptiveAvgPool2d.
        // We choose a kernel and stride that produces the same output dimensions (8x8)
        // as your original adaptive pool layer intended.
        let pool = MaxPool2dConfig::new([3, 3])
            .with_strides([3, 3])
            .init(); // MODIFIED

        let dropout = DropoutConfig::new(self.drop_prob).init();
        // The linear layer input size is still correct because our new pool layer
        // maintains the same output dimensions.
        let linear1 = LinearConfig::new(16 * 8 * 8, self.hidden_size).init(device);
        let linear2 = LinearConfig::new(self.hidden_size, self.num_classes).init(device);
        let activation = Relu::new();
        Model {
            conv1,
            conv2,
            pool,
            dropout,
            linear1,
            linear2,
            activation,
        }
    }
}

// No changes are needed below this line
// =====================================

impl<B: Backend> Model<B> {
    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 2> {
        let [batch_size, height, width] = x.dims();
        let x = x.reshape([batch_size, 1, height, width]);

        let x = self.conv1.forward(x);
        let x = self.dropout.forward(x);
        let x = self.conv2.forward(x);
        let x = self.dropout.forward(x);
        let x = self.activation.forward(x);

        let x = self.pool.forward(x); // This now calls the supported MaxPool2d layer
        let x = x.reshape([batch_size, 16 * 8 * 8]);
        let x = self.linear1.forward(x);
        let x = self.dropout.forward(x);
        let x = self.activation.forward(x);

        self.linear2.forward(x)
    }

    pub fn forward_classification(
        &self,
        images: Tensor<B, 3>,
        targets: Tensor<B, 1, Int>,
    ) -> ClassificationOutput<B> {
        let output = self.forward(images);
        let loss = CrossEntropyLossConfig::new()
            .init(&output.device())
            .forward(output.clone(), targets.clone());

        ClassificationOutput {
            loss,
            output,
            targets,
        }
    }
}

impl<B: AutodiffBackend> TrainStep<MnistBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, item: MnistBatch<B>) -> burn::train::TrainOutput<ClassificationOutput<B>> {
        let item = self.forward_classification(item.images, item.targets);
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<MnistBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, item: MnistBatch<B>) -> ClassificationOutput<B> {
        self.forward_classification(item.images, item.targets)
    }
}