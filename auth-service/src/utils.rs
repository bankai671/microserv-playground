use argon2;
use rand::Rng;
use jsonwebtoken::{
    Header,
    EncodingKey,
    DecodingKey,
    Validation,
};
use crate::model::TokenClaims;
use redis::{Client, RedisResult, aio::MultiplexedConnection};
use std::sync::Arc;
use chrono::{Utc, Duration};

pub struct RedisStore {
    client: Arc<Client>,
}

impl RedisStore {
    pub async fn new(redis_url: &str) -> Self {
        let client = Client::open(redis_url).expect("Invalid Redis URL");
        RedisStore {
            client: Arc::new(client),
        }
    }

    async fn get_connection(&self) -> RedisResult<MultiplexedConnection> {
        self.client.get_multiplexed_async_connection().await
    }

    pub async fn set(&self, key: &str, value: &str, exp_time: usize) -> RedisResult<()> {
        let mut con = self.get_connection().await?;
        redis::cmd("SETEX")
            .arg(key)
            .arg(exp_time)
            .arg(value)
            .query_async(&mut con)
            .await
    }

    pub async fn get(&self, key: &str) -> RedisResult<String> {
        let mut con = self.get_connection().await?;
        redis::cmd("GET")
            .arg(key)
            .query_async(&mut con)
            .await
    }

    pub async fn delete(&self, key: &str) -> RedisResult<()> {
        let mut con = self.get_connection().await?;
        redis::cmd("DEL").arg(key).query_async(&mut con).await
    }
}

pub fn hash_password (password: &str) -> Result<String, argon2::Error> {
    let salt: [u8; 16] = rand::thread_rng().gen();

    argon2::hash_encoded(password.as_bytes(), &salt, &argon2::Config {
        variant: argon2::Variant::Argon2id,
        version: argon2::Version::Version13,
        mem_cost: 32768,
        time_cost: 1,
        lanes: 8,
        ..Default::default()
    })
}

pub fn verify_password (hash: &str, password: &str) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password.as_bytes())
}

pub fn generate_access_token(uid: &str, jwt_secret: &str, exp_time_sec: &str) -> Result<String, String> {
    let exp_duration = exp_time_sec.parse::<i64>()
        .map_err(|_| "Failed to parse expiration time".to_string())?;

    let exp = (Utc::now() + Duration::seconds(exp_duration)).timestamp() as usize;

    let claims = TokenClaims {
        sub: uid.to_string(),
        exp,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes())
    ).map_err(|e| format!("Failed to generate access token: {}", e))
}

pub fn generate_refresh_token(uid: &str, jwt_secret: &str, exp_time_sec: &str) -> Result<String, String> {
    let exp_duration = exp_time_sec.parse::<i64>()
        .map_err(|_| "Failed to parse expiration time".to_string())?;

    let exp = (Utc::now() + Duration::seconds(exp_duration)).timestamp() as usize;

    let claims = TokenClaims {
        sub: uid.to_string(),
        exp,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes())
    ).map_err(|e| format!("Failed to generate refresh token: {}", e))
}

pub fn decode_token(token: &str, jwt_secret: &str) -> Result<jsonwebtoken::TokenData<TokenClaims>, String> {
    jsonwebtoken::decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ).map_err(|e| format!("Failed to decode token: {}", e))
}
