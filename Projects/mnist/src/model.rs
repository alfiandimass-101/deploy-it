use burn::{backend::Candle, nn::{conv::Conv2d, pool::AdaptiveAvgPool2d, Dropout}, prelude::*};

#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    pool: AdaptiveAvgPool2d,
    dropout: Dropout,
    linear1: Linear<B>,
    linear2: Linear<B>,
}