#[cfg(test)]
mod tests {
    use polars::io::{csv::CsvReader, SerReader};

    #[test]
    fn test_load_dataset() -> anyhow::Result<()> {
        let bj_data = CsvReader::from_path("resources/ch04_beijing_house_price.csv.zst")?
            .has_header(true)
            .finish()?;

        println!("{}", bj_data.head(Some(5)));

        Ok(())
    }
}
