use burn::{
    backend::{
        wgpu::{AutoGraphicsApi, WgpuDevice},
        Autodiff, Wgpu,
    },
    data::dataset::{source::huggingface::MNISTDataset, Dataset},
    optim::AdamConfig,
};

use crate::model::ModelConfig;

mod data;
mod infer;
mod model;
mod train;

fn main() {
    type MyBackend = Wgpu<AutoGraphicsApi, f32, i32>;
    type MyAutoDiffBackend = Autodiff<MyBackend>;

    let artifact_dir = "/tmp/guide";
    let device = WgpuDevice::default();

    train::train::<MyAutoDiffBackend>(
        artifact_dir,
        train::TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
        device.clone(),
    );

    infer::infer::<MyAutoDiffBackend>(
        artifact_dir,
        device.clone(),
        MNISTDataset::test().get(42).unwrap(),
    )
}
