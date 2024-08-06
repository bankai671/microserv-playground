use axum::{
    routing::get,
    Router
};

use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let num: usize = num_cpus::get();

    let app = Router::new()
        .route("/", get(root));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    info!("Server is running on port 3001!, with {num} cpus");
    axum::serve(listner, app).await.unwrap();
}

async fn root () -> &'static str {
    "hello from / endpoint\n"
}
