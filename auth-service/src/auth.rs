use axum::{
    Json,
    http::StatusCode
};
use argon2::{self, Config};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::sync::LazyLock;

#[derive(Deserialize, Debug)]
pub struct RegisterPayload {
    email: String,
    username: String,
    password: String,
    confirm_password: String
}

#[derive(Serialize, Debug)]
pub struct RegisterPayloadReturn {
    uid: u32,
    email: String,
    username: String,
    password: String
}

#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    message: String
}

static ARGON2_CFG: LazyLock<Config> = LazyLock::new(|| Config {
    variant: argon2::Variant::Argon2id,
    version: argon2::Version::Version13,
    mem_cost: 32768,
    time_cost: 1,
    lanes: 8,
    ..Default::default()
});

pub async fn register (
Json(payload): Json<RegisterPayload>
) -> Result<(StatusCode, Json<RegisterPayloadReturn>), (StatusCode, Json<ErrorMessage>)> {
    if payload.password != payload.confirm_password {
        let err_msg = ErrorMessage {
            message: "password not match with confirm_password".to_string()
        };

        return Err((StatusCode::UNAUTHORIZED, Json(err_msg)));
    };
    
    // TODO:
    // check if user exist in user-service
    // if exist return 409 and ErrorMessage

    let salt: [u8; 16] = rand::thread_rng().gen();

    
    let encrypted = argon2::hash_encoded(payload.password.as_bytes(), &salt, &ARGON2_CFG).unwrap();
    
    let res_json = RegisterPayloadReturn {
                    uid: 1,
                    email: payload.email.to_string(),
                    username: payload.username.to_string(),
                    password: encrypted
    };

    Ok((StatusCode::OK, Json(res_json)))
}
