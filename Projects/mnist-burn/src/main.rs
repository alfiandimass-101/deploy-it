use burn::{backend::{Autodiff, Candle}, nn::{conv::{Conv2d, Conv2dConfig}, pool::{MaxPool2d, MaxPool2dConfig}, Linear, LinearConfig, Relu}, prelude::*, tensor::{loss::cross_entropy_with_logits, T}, train::ClassificationOutput};
use burn::data::{dataloader::DataLoaderBuilder, dataset::vision::MnistDataset};
use burn_dataset::vision::MnistItem;


// Backend for MNIST model (using cpu with candle)
// f32 for calculation and i64 for the dataset label
type MyBackend = Autodiff<Candle<f32, i64>>;

// Model for MNIST (as the brain)
#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    pool: MaxPool2d,
    relu: Relu,
    linear1: Linear<B>, // input
    linear2: Linear<B>, //output
}

impl<B: Backend> Model<B> {
    // create new model
    pub fn new(num_class: usize) -> Self {
        // configuration of the layers.
        let device = &Default::default();
        let conv1 = Conv2dConfig::new([1, 8], [3,3]).init(device); // 1 input(gambar) -> 8 output dengan 3x3 kernel.
        let conv2 = Conv2dConfig::new([8, 16], [3,3]).init(device); // 8 input -> 16 output dengan 3x3 kernel.
        let pool = MaxPool2dConfig::new([2,2]).with_strides([2,2]).init(); // kernel 2x2 untuk pooling.
        let relu = Relu::new(); // aktivasi untuk non-linear
        let linear1 = LinearConfig::new(16*5*5, 128).init(device); // input hasil flatten conv, output 128 neuron
        let linear2 = LinearConfig::new(128, num_class).init(device); // Input 128, output sejumlah kelas(ex: 10)
        Self { conv1, conv2, pool, relu, linear1, linear2 } 
    }

    pub fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 2> {
        // Input: [batch_size, 1, 28, 28]
        let x = self.conv1.forward(x); // -> [batch_size, 8, 26, 26]
        let x = self.relu.forward(x);
        let x = self.pool.forward(x); // -> [batch_size, 8, 13, 13]

        let x = self.conv2.forward(x); // -> [batch_size, 16, 11, 11]
        let x = self.relu.forward(x);
        let x = self.pool.forward(x); // -> [batch_size, 16, 5, 5]

        // Flatten data dari 4D ke 2D untuk masuk ke layer Linear.
        let [batch_size, _, _, _] = x.dims();
        let x = x.reshape([batch_size, 16*5*5]); // -> [batch_size, 400]

        let x = self.linear1.forward(x); // -> [batch_size, 128]
        let x = self.relu.forward(x);

        // Output (Logits)
        self.linear2.forward(x) // -> [batch_size, num_class]
    }

    pub fn forward_classification(&self, item: MnistItem) -> ClassificationOutput<B> {
        let targets = Tensor::<B, 2>::from_data([1, item.label].into(), &Default::default());
        let output = self.forward(item.image.into());
        // using burn efficient loss cross-entropy function.
        let loss = cross_entropy_with_logits(output, targets);
        ClassificationOutput { loss, output, targets }
    }
}

fn main() {
    println!("Hello, world!");
}
