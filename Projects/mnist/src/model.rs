use burn::{nn::{conv::{Conv1dConfig, Conv2d, Conv2dConfig}, pool::{AdaptiveAvgPool2d, AdaptiveAvgPool2dConfig}, Dropout, DropoutConfig, Linear, LinearConfig, Relu}, prelude::*};

#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    pool: AdaptiveAvgPool2d,
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
    drop_prob: f64
}

impl ModelConfig {
    pub fn init<B: Backend>(&self) -> Model<B> {
        let device = &Default::default();
        let conv1 = Conv2dConfig::new([1,8], [3,3]).init(device);
        let conv2 = Conv2dConfig::new([8, 16], [3,3]).init(device);
        let pool = AdaptiveAvgPool2dConfig::new([8,8]).init();
        let dropout = DropoutConfig::new(self.drop_prob).init();
        let linear1 = LinearConfig::new(16*8*8, self.hidden_size).init(device);
        let linear2 = LinearConfig::new(self.hidden_size, self.num_classes).init(device);
        let activation = Relu::new();
        Model { conv1, conv2, pool, dropout, linear1, linear2, activation }
    }
}

impl<B: Backend> Model<B> {
    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 2> {
        let [batch_size, height, width] = x.dims();
        let x = x.reshape([batch_size, 1, height, width]);

        let x = self.conv1.forward(x);
        let x = self.dropout.forward(x);
        let x = self.conv2.forward(x);
        let x = self.dropout.forward(x);
        let x = self.activation.forward(x);

        let x = self.pool.forward(x);
        let x = x.reshape([batch_size, 16*8*8]);
        let x = self.linear1.forward(x);
        let x = self.dropout.forward(x);
        let x = self.activation.forward(x);

        self.linear2.forward(x)
    }
}