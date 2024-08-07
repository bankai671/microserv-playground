use axum::{
    routing::{
        get,
        post,
    },
    Router,
};

use dotenv::dotenv;

mod auth;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let host_name = std::env::var("HOST_URL").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("3001".to_string());

    tracing_subscriber::fmt::init();

    let app: _ = Router::new()
        .route("/", get(|| async {
            "Hello from / endpoint"
        }))
        .route("/register", post(auth::register));

    let listener = tokio::net::TcpListener::bind(format!("{host_name}:{port}")).await.unwrap();

    axum::serve(listener, app).await.unwrap()
}

