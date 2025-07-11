// src/data.rs
use burn::data::dataset::{HuggingfaceDatasetLoader, Dataset};
use burn::prelude::*;
use serde::{Deserialize, Serialize};

// Nama field 'img' dan 'label' harus cocok dengan nama kolom di dataset Hugging Face
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Cifar10Item {
    pub img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, // Gunakan Rgb untuk gambar berwarna
    pub label: u32,
}

// Batcher untuk mengubah Cifar10Item menjadi Cifar10Batch berisi Tensor
pub struct Cifar10Batcher<B: Backend> {
    device: B::Device,
}

impl<B: Backend> Cifar10Batcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

impl<B: Backend> burn::data::dataloader::batcher::Batcher<Cifar10Item, Cifar10Batch<B>> for Cifar10Batcher<B> {
    fn batch(&self, items: Vec<Cifar10Item>) -> Cifar10Batch<B> {
        let images = items
            .iter()
            .map(|item| {
                // Konversi gambar ke tensor
                let mut data = Data::<f32, 3>::zeros([3, 32, 32]);
                for (x, y, pixel) in item.img.enumerate_pixels() {
                    let [r, g, b] = pixel.0;
                    data.set([0, y as usize, x as usize], (r as f32) / 255.0 - 0.5);
                    data.set([1, y as usize, x as usize], (g as f32) / 255.0 - 0.5);
                    data.set([2, y as usize, x as usize], (b as f32) / 255.0 - 0.5);
                }
                Tensor::<B, 3>::from_data(data, &self.device)
            })
            .collect();

        let targets = items
            .iter()
            .map(|item| Tensor::<B, 1, Int>::from_data(Data::from([(item.label as i64)]), &self.device))
            .collect();

        let images = Tensor::cat(images, 0).reshape([-1, 3, 32, 32]);
        let targets = Tensor::cat(targets, 0);

        Cifar10Batch { images, targets }
    }
}

#[derive(Clone, Debug)]
pub struct Cifar10Batch<B: Backend> {
    pub images: Tensor<B, 4>,
    pub targets: Tensor<B, 1, Int>,
}

// Fungsi untuk memuat dataset
pub fn load_cifar10_dataset(split: &str) -> impl Dataset<Cifar10Item> {
    HuggingfaceDatasetLoader::new("cifar10")
        .load(split)
        .unwrap()
}