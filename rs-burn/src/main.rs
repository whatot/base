use burn::{
    backend::{
        wgpu::{AutoGraphicsApi, WgpuDevice},
        Autodiff, Wgpu,
    },
    optim::AdamConfig,
};

use crate::model::ModelConfig;

mod data;
mod model;
mod train;

fn main() {
    type MyBackend = Wgpu<AutoGraphicsApi, f32, i32>;
    type MyAutoDiffBackend = Autodiff<MyBackend>;

    let device = WgpuDevice::default();
    train::train::<MyAutoDiffBackend>(
        "/tmp/guide",
        train::TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
        device,
    );
}
