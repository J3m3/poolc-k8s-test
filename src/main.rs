use axum::{http::StatusCode, routing::get, Router};

type Result<T> = core::result::Result<T, Box<dyn core::error::Error + 'static>>;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/health", get(|| async { StatusCode::OK }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    Ok(axum::serve(listener, app).await?)
}
