use ndarray::{Array1, Array2, Axis};
use ndarray_linalg::Inverse;

// ndarray
// https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html
// https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html#method.mapv

/// 最小二乘法代数求解
#[allow(dead_code)]
pub fn least_squares_algebraic(x: &Array1<f64>, y: &Array1<f64>) -> (f64, f64) {
    let n = x.len_of(Axis(0)) as f64;
    let w1 = (n * (x * y).sum() - x.sum() * y.sum()) as f64
        / (n * (x * x).sum() - x.sum() * x.sum()) as f64;
    let w0 = ((x * x).sum() * y.sum() - x.sum() * (x * y).sum()) as f64
        / (n * (x * x).sum() - x.sum() * x.sum()) as f64;
    return (w0, w1);
}

/// 平方损失函数
#[allow(dead_code)]
pub fn square_loss(x: &Array1<f64>, y: &Array1<f64>, w0: f64, w1: f64) -> f64 {
    (y.mapv(|yv| yv as f64) - x.mapv(|value| w0 + w1 * value as f64))
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
    use linfa::prelude::Predict;
    use linfa::traits::Fit;
    use linfa::Dataset;
    use ndarray::array;
    use ndarray::concatenate;
    use ndarray::Array;
    use ndarray::Array1;
    use ndarray::Array2;
    use ndarray::Axis;

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
}
