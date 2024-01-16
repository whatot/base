#[cfg(test)]
mod tests {

    use anyhow::Ok;
    use linfa::traits::Fit;

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
        todo!()
    }

    #[test]
    fn test_solve_regression_by_matrix() -> anyhow::Result<()> {
        todo!()
    }
}
