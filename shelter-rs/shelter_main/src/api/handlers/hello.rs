use axum::http::StatusCode;

pub async fn hello() -> Result<String, StatusCode> {
    Ok("\nHello World\n\n".to_string())
}
