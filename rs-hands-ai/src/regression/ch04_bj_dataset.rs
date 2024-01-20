#[cfg(test)]
mod tests {
    use linfa::traits::Fit;
    use polars::io::{csv::CsvReader, SerReader};

    use crate::assert_approx_eq;
    use crate::tools::SplitDataset;
    use linfa::prelude::Predict;
    use linfa::prelude::SingleTargetRegression;

    #[test]
    fn test_load_dataset() -> anyhow::Result<()> {
        let bj_data = CsvReader::from_path("resources/ch04_beijing_house_price.csv.zst")?
            .has_header(true)
            .finish()?;

        // 通过源码分析，linfa判定矩阵不可逆的逻辑有问题，所以需要去除一列全为0的特征(商场)
        // https://github.com/rust-ml/linfa-linalg/blob/main/src/qr.rs#solve_into
        let feature_names = vec![
            "公交",
            "写字楼",
            "医院",
            "地铁",
            "学校",
            "建造时间",
            "楼层",
            "面积",
        ];
        let target_name = "每平米价格";
        let split_dataset = SplitDataset::new(&bj_data, feature_names, target_name, 0.7f32)?;
        let dataset_train = split_dataset.train_dataset();
        let dataset_test = split_dataset.test_dataset();

        let lin_reg = linfa_linear::LinearRegression::new();
        let model = lin_reg.fit(&dataset_train)?;
        // dbg!(&model);

        let predict_result = model.predict(&dataset_test);

        // Mean absolute percentage error between two continuous variables
        // MAPE = 1/N * SUM(abs((y_hat - y) / y))
        let mape = predict_result.mean_absolute_percentage_error(&dataset_test)?;
        // dbg!(&mape);
        assert_approx_eq!(mape, 0.5482529f64);

        Ok(())
    }
}
