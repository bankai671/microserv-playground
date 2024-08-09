#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub jwt_secret: String,
    pub redis_port: String
}

impl Config {
    pub fn init() -> Self {
        let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
        let port = std::env::var("PORT").unwrap_or("8009".to_string());
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or("ebal rot etogo rusta".to_string());
        let redis_port = std::env::var("REDIS_PORT").unwrap_or("6379".to_string());

        Self {
            host,
            port,
            jwt_secret,
            redis_port
        }
    }
}
