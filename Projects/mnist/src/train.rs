use burn::{data::{dataloader::DataLoaderBuilder, dataset::vision::MnistDataset}, optim::AdamConfig, prelude::*, record::CompactRecorder, tensor::backend::AutodiffBackend, train::{metric::{AccuracyMetric, LossMetric}, LearnerBuilder}};
use crate::{data::MnistBatcher, model::ModelConfig};

#[derive(Config)]
pub struct TrainingConfig {
    pub model: ModelConfig,
    pub optimizer: AdamConfig,
    #[config(default = 10)]
    pub num_epochs: usize,
    #[config(default = 64)]
    pub batch_size: usize,
    #[config(default = 4)]
    pub num_workers: usize,
    #[config(default = 42)]
    pub seed: u64,
    #[config(default = 1.0e-4)]
    pub learning_rate: f64,
}

fn create_artifact_dir(artifact_dir: &str) {
    std::fs::remove_dir_all(artifact_dir).ok();
    std::fs::create_dir_all(artifact_dir).ok();
}

pub fn train<B: AutodiffBackend>(artifact_dir: &str, config: TrainingConfig, device: B::Device) {
    create_artifact_dir(artifact_dir);
    config.save(format!("{artifact_dir}/config.json"))
    .expect("Config ga tersimpan");

    B::seed(config.seed);

    let batcher = MnistBatcher::default();

    let dataloader_train = DataLoaderBuilder::new(batcher.clone())
    .batch_size(config.batch_size)
    .num_workers(config.num_workers)
    .shuffle(config.seed)
    .build(MnistDataset::train());

    let dataloader_test = DataLoaderBuilder::new(batcher)
    .batch_size(config.batch_size)
    .num_workers(config.num_workers)
    .shuffle(config.seed)
    .build(MnistDataset::test());

    let learner = LearnerBuilder::new(artifact_dir)
        .metric_train_numeric(AccuracyMetric::new())
        .metric_valid_numeric(AccuracyMetric::new())
        .metric_train_numeric(LossMetric::new())
        .metric_valid_numeric(LossMetric::new())
        .with_file_checkpointer(CompactRecorder::new())
        .devices(vec![device.clone()])
        .num_epochs(config.num_epochs)
        .summary()
        .build(
            config.model.init::<B>(),
            config.optimizer.init(),
            config.learning_rate,
        );

    let model_trained = learner.fit(dataloader_train, dataloader_test);

    model_trained
        .save_file(format!("{artifact_dir}/model"), &CompactRecorder::new())
        .expect("Trained model should be saved successfully");
}