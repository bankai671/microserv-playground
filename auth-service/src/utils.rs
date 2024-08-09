use argon2;
use rand::{
    Rng,
    distributions::Alphanumeric
};
use jsonwebtoken::{
    Header,
    EncodingKey,
};
use uuid::Uuid;

use crate::model::TokenClaims;

pub fn hash_password (password: &str) -> Result<String, argon2::Error> {
    let salt: [u8; 16] = rand::thread_rng().gen();

    argon2::hash_encoded(password.as_bytes(), &salt, &argon2::Config{
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

pub async fn generate_access_token () -> String {
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::hours(4)).timestamp() as usize;
    
    let sub = Uuid::new_v4();

    let claims = TokenClaims {
        sub: sub.to_string(),
        exp
    };

    let jwt_secret = std::env::var("JWT_SECRET").unwrap();

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref())
    ).unwrap();
    
    token
}

pub async fn generate_refresh_token () -> String {
    Uuid::new_v4().to_string()
}
