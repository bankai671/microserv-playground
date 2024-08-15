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
use reqwest::Client as HttpClient;
use crate::utils::RedisStore;

pub struct AppState {
    pub env: Config,
    pub http_client: HttpClient,
    pub redis_store: RedisStore
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let config = Config::init();
    let http_client = HttpClient::new();
    let redis_store = RedisStore::new(&config.redis_url).await;

    let app_state = Arc::new(AppState {
        env: config.clone(),
        http_client,
        redis_store
    });
    
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let app = Router::new()
        .route("/", get(|| async {
            "Hello from auth-service / endpoint, port 8001"
        }))
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/refresh-token", post(auth::refresh_token))
        .with_state(app_state);
    
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("Server is started and listening port: {}", config.port);

    axum::serve(listener, app).await.unwrap()
}

