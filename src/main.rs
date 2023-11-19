use axum::routing::{get, Router};
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 3030;
    let addr = format!{"0.0.0.0:{}", port};

    let app = Router::new()
        .route("/", get(handlers::health))
        .route("/capacity", get(handlers::capacity));

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
