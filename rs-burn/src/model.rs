use burn::{
    config::Config,
    module::Module,
    nn::{
        conv::{Conv2d, Conv2dConfig},
        loss::CrossEntropyLoss,
        pool::{AdaptiveAvgPool2d, AdaptiveAvgPool2dConfig},
        Dropout, DropoutConfig, Linear, LinearConfig, ReLU,
    },
    tensor::{
        backend::{AutodiffBackend, Backend},
        Int, Tensor,
    },
    train::{ClassificationOutput, TrainOutput, TrainStep, ValidStep},
};

use crate::data::MNISTBatch;

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    pool: AdaptiveAvgPool2d,
    dropout: Dropout,
    linear1: Linear<B>,
    linear2: Linear<B>,
    activation: ReLU,
}

#[derive(Config, Debug)]
pub struct ModelConfig {
    num_classes: usize,
    hidden_classes: usize,
    #[config(default = "0.5")]
    dropout: f64,
}

impl ModelConfig {
    /// Returns the initialized model.
    pub fn init<B: Backend>(&self) -> Model<B> {
        Model {
            conv1: Conv2dConfig::new([1, 8], [3, 3]).init(),
            conv2: Conv2dConfig::new([8, 16], [3, 3]).init(),
            pool: AdaptiveAvgPool2dConfig::new([8, 8]).init(),
            dropout: DropoutConfig::new(self.dropout).init(),
            linear1: LinearConfig::new(16 * 8 * 8, self.hidden_classes).init(),
            linear2: LinearConfig::new(self.hidden_classes, self.num_classes).init(),
            activation: ReLU::new(),
        }
    }
}

impl<B: Backend> Model<B> {
    /// # Shapes
    ///   - Images [batch_size, height, width]
    ///   - Output [batch_size, num_classes]
    pub fn forward(&self, images: Tensor<B, 3>) -> Tensor<B, 2> {
        let [batch_size, height, weight] = images.dims();

        // Create a channel at the second dimension.
        let x = images.reshape([batch_size, 1, height, weight]);

        // [batch_size, 8, _, _]
        let x = self.conv1.forward(x);
        let x = self.dropout.forward(x);
        // [batch_size, 16, _, _]
        let x = self.conv2.forward(x);
        let x = self.dropout.forward(x);
        let x = self.activation.forward(x);

        // [batch_size, 16, 8, 8]
        let x = self.pool.forward(x);
        let x = x.reshape([batch_size, 16 * 8 * 8]);
        let x = self.linear1.forward(x);
        let x = self.dropout.forward(x);
        let x = self.activation.forward(x);

        // [batch_size, num_classes]
        self.linear2.forward(x)
    }

    pub fn forward_classification(
        &self,
        images: Tensor<B, 3>,
        targets: Tensor<B, 1, Int>,
    ) -> ClassificationOutput<B> {
        let output = self.forward(images);
        let loss = CrossEntropyLoss::new(None).forward(output.clone(), targets.clone());

        ClassificationOutput::new(loss, output, targets)
    }
}

impl<B: AutodiffBackend> TrainStep<MNISTBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, batch: MNISTBatch<B>) -> burn::train::TrainOutput<ClassificationOutput<B>> {
        let item = self.forward_classification(batch.images, batch.targets);
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<MNISTBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, batch: MNISTBatch<B>) -> ClassificationOutput<B> {
        self.forward_classification(batch.images, batch.targets)
    }
}
