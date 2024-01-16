use burn::{
    config::Config,
    data::{dataloader::batcher::Batcher, dataset::source::huggingface::MNISTItem},
    module::Module,
    record::{CompactRecorder, Recorder},
    tensor::backend::Backend,
};

use crate::{data::MNISTBatcher, train::TrainingConfig};

pub fn infer<B: Backend>(artifact_dir: &str, device: B::Device, item: MNISTItem) {
    let config = TrainingConfig::load(format!("{artifact_dir}/config.json"))
        .expect("Config should exist for the model");
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into())
        .expect("Trained model should exist");

    let model = config.model.init_with::<B>(record).to_device(&device);

    let label = item.label;
    let batcher = MNISTBatcher::new(device);
    let batch = batcher.batch(vec![item]);
    let output = model.forward(batch.images);
    let predicted = output.argmax(1).flatten::<1>(0, 1).into_scalar();
    println!("Predicted {} Expected {}", predicted, label);
}
