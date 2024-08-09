mod auth;
mod model;
mod utils;
mod config;

use axum::{
    routing::{
        get,
        post,
    },
    Router,
};
use dotenv::dotenv;
use config::Config;
use std::sync::Arc;

pub struct AppState {
    pub env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let config = Config::init();

    let app_state = Arc::new(AppState {
        env: config.clone()
    });

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async {
            "Hello from / endpoint"
        }))
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .with_state(app_state);
    
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("Server is started and listening port: {}", config.port);

    axum::serve(listener, app).await.unwrap()
}

