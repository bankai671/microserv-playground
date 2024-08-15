#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub jwt_secret: String,
    pub redis_url: String,
    pub user_service_url: String,
    pub jwt_access_exp_time_sec: String,
    pub jwt_refresh_exp_time_sec: String
}

impl Config {
    pub fn init() -> Self {
        let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
        let port = std::env::var("PORT").unwrap_or("8009".to_string());
        let user_service_url = std::env::var("USER_SERVICE_URL").unwrap_or("http://user-service:8002".to_string());
        let redis_url = std::env::var("REDIS_URL").unwrap_or("redis://redis:6379".to_string());
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or("secret".to_string());
        let jwt_access_exp_time_sec = std::env::var("JWT_ACCESS_EXP_TIME_SEC").unwrap_or("900".to_string());
        let jwt_refresh_exp_time_sec = std::env::var("JWT_REFRESH_EXP_TIME_SEC").unwrap_or("604800".to_string());

        Self {
            host,
            port,
            user_service_url,
            redis_url,
            jwt_secret,
            jwt_access_exp_time_sec,
            jwt_refresh_exp_time_sec
        }
    }
}
