use axum::{
    Json,
    http::StatusCode
};
use crate::utils::{self};
use crate::model::{RegisterDto, LoginDto, LoginResponse, ErrorMessage};

pub async fn register (
    Json(payload): Json<RegisterDto>
)-> StatusCode
{
    if payload.password != payload.confirm_password {
        return StatusCode::UNAUTHORIZED;
    };
    
    // TODO:
    // check if user exist in user-service
    // if exist return 409
    // let hashed_password = utils::hash_password(&payload.password).unwrap();
    // call user-service set user into db with hashed_password
 
    StatusCode::OK
}

pub async fn login (
    Json(_payload): Json<LoginDto>
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, Json<ErrorMessage>)> {
    // TODO:
    // check if user is already logged in (store user from user-service in variable user to use it later)
    // compare password and hash
    // in future add email verefication

    let access_token = utils::generate_access_token().await;
    let refresh_token = utils::generate_refresh_token().await;

    // save token to redis - (key) token - (value) user_id

    let response_json = LoginResponse {
        access_token: access_token.to_string(),
        refresh_token: refresh_token.to_string()
    };

    Ok((StatusCode::OK, Json(response_json)))
}

