use ndarray::{Array1, Array2, Axis};
use ndarray_linalg::Inverse;

// ndarray
// https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html
// https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html#method.mapv

/// 最小二乘法代数求解
#[allow(dead_code)]
pub fn least_squares_algebraic(x: &Array1<f64>, y: &Array1<f64>) -> (f64, f64) {
    let n = x.len_of(Axis(0)) as f64;
    let w1 = (n * (x * y).sum() - x.sum() * y.sum()) / (n * (x * x).sum() - x.sum() * x.sum());
    let w0 = ((x * x).sum() * y.sum() - x.sum() * (x * y).sum())
        / (n * (x * x).sum() - x.sum() * x.sum());
    return (w0, w1);
}

/// 平方损失函数
#[allow(dead_code)]
pub fn square_loss(x: &Array1<f64>, y: &Array1<f64>, w0: f64, w1: f64) -> f64 {
    (y - x.mapv(|value| w0 + w1 * value))
        .mapv(|value| value * value)
        .sum()
}

/// 最小二乘法矩阵求解
///  (x.T * x).I * x.T * y
#[allow(dead_code)]
pub fn least_squares_matrix(x: &Array2<f64>, y: &Array2<f64>) -> anyhow::Result<(f64, f64)> {
    // 计算x的转置乘以x
    let xtx = x.t().dot(x);

    // 计算x的逆
    let inv_xtx = xtx.inv()?;

    // 计算x的转置乘以y
    let xt_y = x.t().dot(y);

    // 最后，计算w = inv_xtx * xt_y
    let w = inv_xtx.dot(&xt_y);

    Ok((w[[0, 0]], w[[1, 0]]))
}

#[cfg(test)]
mod tests {

    use crate::assert_approx_eq;
    use crate::regression::ch03_basic_regression::least_squares_matrix;

    use super::least_squares_algebraic;
    use super::square_loss;
    use anyhow::Ok;
    use linfa::prelude::*;
    use linfa::{traits::Fit, Dataset};
    use ndarray::{array, concatenate, Array, Array1, Array2, Axis};
    use polars::prelude::*;

    const X_ARRAY: [f64; 10] = [56., 72., 69., 88., 102., 86., 76., 79., 94., 74.];
    const Y_ARRAY: [f64; 10] = [92., 102., 86., 110., 130., 99., 96., 102., 105., 92.];
    const EXPECT_W0: f64 = 41.33509168550616f64;
    const EXPECT_W1: f64 = 0.7545842753077117f64;
    const EXPECT_LOSS: f64 = 447.69153479025357f64;

    #[test]
    fn test_solve_by_linfa() -> anyhow::Result<()> {
        let x = Array::from_shape_vec((X_ARRAY.len(), 1), X_ARRAY.to_vec())?;
        let y = Array1::from_iter(Y_ARRAY);
        let dataset = Dataset::new(x, y);

        let lin_reg = linfa_linear::LinearRegression::new();
        let model = lin_reg.fit(&dataset)?;

        assert_approx_eq!(model.intercept(), EXPECT_W0);
        assert_approx_eq!(model.params()[0], EXPECT_W1);

        let expect_150_result = 154.5227329816629f64;

        // predict y when x=150
        let predict_x = Array2::from_elem((1, 1), 150.0f64);
        let predict_result = model.predict(&predict_x);
        assert_approx_eq!(predict_result[0], expect_150_result);

        // predict multi x
        let predict_multi = array![[150.0], [200.0], [300.0]];
        let predict_result = model.predict(&predict_multi);
        assert_eq!(predict_result.len(), 3);
        assert_approx_eq!(predict_result[0], expect_150_result);

        Ok(())
    }

    #[test]
    fn test_solve_by_algebra() -> anyhow::Result<()> {
        let x = Array1::from_iter(X_ARRAY);
        let y = Array1::from_iter(Y_ARRAY);

        let (w0, w1) = least_squares_algebraic(&x, &y);
        let loss = square_loss(&x, &y, w0, w1);
        // println!("w0:{} w1:{} loss:{}", w0, w1, loss);

        assert_approx_eq!(w0, EXPECT_W0);
        assert_approx_eq!(w1, EXPECT_W1);
        assert_approx_eq!(loss, EXPECT_LOSS);

        Ok(())
    }

    #[test]
    fn test_solve_by_matrix() -> anyhow::Result<()> {
        let x_matrix = concatenate![
            Axis(1),
            Array::ones((X_ARRAY.len(), 1)),
            Array::from_shape_vec((X_ARRAY.len(), 1), X_ARRAY.to_vec())?
        ];
        assert_eq!(2, x_matrix.ndim());
        assert_eq!(10, x_matrix.len_of(Axis(0)));
        assert_eq!(2, x_matrix.len_of(Axis(1)));

        let y_matrix = Array::from_shape_vec((Y_ARRAY.len(), 1), Y_ARRAY.to_vec())?;
        assert_eq!(2, y_matrix.ndim());
        assert_eq!(10, y_matrix.len_of(Axis(0)));
        assert_eq!(1, y_matrix.len_of(Axis(1)));

        let (w0, w1) = least_squares_matrix(&x_matrix, &y_matrix)?;
        assert_approx_eq!(w0, EXPECT_W0);
        assert_approx_eq!(w1, EXPECT_W1);

        Ok(())
    }

    // polars
    // https://docs.pola.rs/user-guide/migration/pandas/

    #[test]
    fn test_boston_dataset() -> anyhow::Result<()> {
        let boston_data = CsvReader::from_path("resources/ch03_boston.csv")?
            .has_header(true)
            .finish()?;

        let features = boston_data.select(&["crim", "rm", "lstat"])?;

        // 目标值数据
        let target_series = boston_data.column("medv")?;
        let targets: Vec<f64> = target_series
            .f64()?
            .into_iter()
            .map(|item| item.unwrap())
            .collect();

        // 得到 70% 位置
        let split_num = (features[0].len() as f32 * 0.7f32) as usize;
        let left_num = features[0].len() - split_num;

        // 训练集特征x,训练集目标y
        let x_train = features
            .head(Some(split_num))
            .to_ndarray::<Float64Type>(IndexOrder::C)?;
        let y_train = Array1::from_iter(
            targets
                .iter()
                .take(split_num)
                .cloned()
                .collect::<Vec<f64>>(),
        );
        let dataset_train = Dataset::new(x_train, y_train);

        // 测试集特征x,测试集目标
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
        let dataset_test = Dataset::new(x_test, y_test);

        let lin_reg = linfa_linear::LinearRegression::new();
        let model = lin_reg.fit(&dataset_train)?;
        // dbg!(&model);

        // y = 0.6997 * x1 + 10.1356 * x2 - 0.2053 * x3
        assert_approx_eq!(model.intercept(), -38.00096988969018f64);
        assert_approx_eq!(model.params()[0], 0.69979497f64);
        assert_approx_eq!(model.params()[1], 10.13564218f64);
        assert_approx_eq!(model.params()[2], -0.20532653f64);

        // 对测试集做预测
        let predict_result = model.predict(&dataset_test);

        // 通过寻找到 SingleTargetRegression，找到下面两个统计函数

        // 均方误差（MSE）= sum(np.square(y_true - y_pred)) / n
        let mean_squared_error = predict_result.mean_squared_error(&dataset_test)?;
        assert_approx_eq!(&mean_squared_error, 303.8331247223676f64);
        // dbg!(&mean_squared_error);

        // 平均绝对误差（MAE）= sum(np.abs(y_true - y_pred)) / n
        let mean_absolute_error = predict_result.mean_absolute_error(&dataset_test)?;
        assert_approx_eq!(&mean_absolute_error, 13.022063072780362f64);
        // dbg!(&mean_absolute_error);

        Ok(())
    }
}
