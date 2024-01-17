use ndarray::{Array1, Axis};

// ndarray
// https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html
// https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html#method.mapv

/// 最小二乘法代数求解
#[allow(dead_code)]
pub fn least_squares_algebraic(x: &Array1<u32>, y: &Array1<u32>) -> (f64, f64) {
    let n = x.len_of(Axis(0)) as u32;
    let w1 = (n * (x * y).sum() - x.sum() * y.sum()) as f64
        / (n * (x * x).sum() - x.sum() * x.sum()) as f64;
    let w0 = ((x * x).sum() * y.sum() - x.sum() * (x * y).sum()) as f64
        / (n * (x * x).sum() - x.sum() * x.sum()) as f64;
    return (w0, w1);
}

/// 平方损失函数
#[allow(dead_code)]
pub fn square_loss(x: &Array1<u32>, y: &Array1<u32>, w0: f64, w1: f64) -> f64 {
    (y.mapv(|yv| yv as f64) - x.mapv(|value| w0 + w1 * value as f64))
        .mapv(|value| value * value)
        .sum()
}

#[cfg(test)]
mod tests {

    use crate::assert_approx_eq;

    use super::least_squares_algebraic;
    use super::square_loss;
    use anyhow::Ok;
    use linfa::traits::Fit;
    use ndarray::array;

    #[test]
    fn test_linfa_linear_regression() -> anyhow::Result<()> {
        let dataset = linfa_datasets::diabetes();

        let lin_reg = linfa_linear::LinearRegression::new();
        let model = lin_reg.fit(&dataset)?;

        println!("intercept:  {}", model.intercept());
        println!("parameters: {}", model.params());

        Ok(())
    }

    #[test]
    fn test_solve_regression_by_algebra() -> anyhow::Result<()> {
        let x = array![56, 72, 69, 88, 102, 86, 76, 79, 94, 74];
        let y = array![92, 102, 86, 110, 130, 99, 96, 102, 105, 92];

        let (w0, w1) = least_squares_algebraic(&x, &y);
        let loss = square_loss(&x, &y, w0, w1);
        // println!("w0:{} w1:{} loss:{}", w0, w1, loss);

        assert_approx_eq!(w0, 41.33509168550616f64);
        assert_approx_eq!(w1, 0.7545842753077117f64);
        assert_approx_eq!(loss, 447.69153479025357f64);

        Ok(())
    }

    #[test]
    fn test_solve_regression_by_matrix() -> anyhow::Result<()> {
        Ok(())
    }
}
