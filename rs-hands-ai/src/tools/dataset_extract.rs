use linfa::{Dataset, DatasetBase};
use ndarray::{Array1, Array2};
use polars::{datatypes::Float64Type, frame::DataFrame, prelude::IndexOrder};

#[derive(Debug)]
pub struct SplitDataset {
    /// 训练集特征x
    pub x_train: Array2<f64>,
    /// 训练集目标y
    pub y_train: Array1<f64>,
    /// 测试集特征x
    pub x_test: Array2<f64>,
    /// 测试集目标y
    pub y_test: Array1<f64>,
}

impl SplitDataset {
    pub fn new(
        df: &DataFrame,
        feature_column_names: Vec<&str>,
        target_column_name: &str,
        split_percent: f32,
    ) -> anyhow::Result<SplitDataset> {
        // 特征数据
        let features = df.select(feature_column_names)?;

        // 目标值数据
        let target_series = df.column(target_column_name)?;
        let targets: Vec<f64> = target_series
            .f64()?
            .into_iter()
            .map(|item| item.unwrap())
            .collect();

        // 得到训练集与测试集的切分位置
        let split_num = (features[0].len() as f32 * split_percent) as usize;
        let left_num = features[0].len() - split_num;

        // 训练集
        let x_train: Array2<f64> = features
            .head(Some(split_num))
            .to_ndarray::<Float64Type>(IndexOrder::C)?;
        let y_train: Array1<f64> = Array1::from_iter(
            targets
                .iter()
                .take(split_num)
                .cloned()
                .collect::<Vec<f64>>(),
        );

        // 测试集
        let x_test = features
            .tail(Some(left_num))
            .to_ndarray::<Float64Type>(IndexOrder::C)?;
        let y_test = Array1::from_iter(
            targets
                .iter()
                .skip(split_num)
                .cloned()
                .collect::<Vec<f64>>(),
        );

        Ok(SplitDataset {
            x_train,
            y_train,
            x_test,
            y_test,
        })
    }
}

impl SplitDataset {
    pub fn train_dataset(&self) -> DatasetBase<Array2<f64>, Array1<f64>> {
        Dataset::new(self.x_train.clone(), self.y_train.clone())
    }

    pub fn test_dataset(&self) -> DatasetBase<Array2<f64>, Array1<f64>> {
        Dataset::new(self.x_test.clone(), self.y_test.clone())
    }
}
