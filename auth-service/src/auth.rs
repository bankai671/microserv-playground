use std::sync::Arc;
use axum::{
    Json,
    http::StatusCode,
    extract::State,
};
use crate::utils;
use crate::model::{
    RegisterDto,
    LoginDto,
    LoginResponse,
    CreateUserRequestDto,
    User
};
use crate::AppState;

pub async fn register (
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RegisterDto>
) -> StatusCode {
    if payload.password != payload.confirm_password {
        return StatusCode::UNAUTHORIZED;
    };

    let hash = match utils::hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(_) => {
            return StatusCode::SERVICE_UNAVAILABLE;
        }
    };

    create_user(State(app_state), &payload.email, &payload.username, &hash).await
}

pub async fn login (
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginDto>
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, String)> {
    let user = fetch_user(State(app_state.clone()), &payload.email).await?;

    let is_correct_username = user.username == payload.username;

    if !is_correct_username {
        return Err((StatusCode::UNAUTHORIZED, "Username not correct!".to_string()))
    }

    let is_correct_pwd = match utils::verify_password(&user.password, &payload.password) {
        Ok(is_ok) => is_ok,
        Err(err) => {
            return Err((StatusCode::SERVICE_UNAVAILABLE, err.to_string()))
        }
    };

    if !is_correct_pwd {
        return Err((StatusCode::UNAUTHORIZED, "Password not correct!".to_string()))  
    };
 
    let access_token = utils::generate_access_token(&user.id.to_string(), &app_state.env.jwt_secret).await;
    let refresh_token = utils::generate_refresh_token(&user.id.to_string(), &app_state.env.jwt_secret).await;

    // save token to redis - (key) token - (value) user_id

    let response_json = LoginResponse {
        access_token: access_token.to_string(),
        refresh_token: refresh_token.to_string()
    };

    Ok((StatusCode::OK, Json(response_json)))
}

async fn create_user(
    State(app_state): State<Arc<AppState>>,
    email: &str,
    username: &str,
    password: &str
) -> StatusCode {
    let client = &app_state.client;
    let create_user_url = format!("{}/users", &app_state.env.user_service_url);

    match client         
        .post(&create_user_url)
        .json(&CreateUserRequestDto {
            email: email.to_string(),
            username: username.to_string(),
            password: password.to_string()
        })
        .send()
        .await {
            Ok(res) => {
                if res.status() == StatusCode::CONFLICT {
                    StatusCode::CONFLICT
                } else {
                    StatusCode::OK
                }
            }
            Err(_) => StatusCode::SERVICE_UNAVAILABLE
        }
}

async fn fetch_user(
    State(app_state): State<Arc<AppState>>,
    email: &str
) -> Result<User, (StatusCode, String)> {
    let client = &app_state.client;
    let base_url = &app_state.env.user_service_url;
    let get_user_url = format!("{}/users?email={}", base_url, email);
   
    let response = match client.get(&get_user_url).send().await {
        Ok(response) => response,
        Err(_) => {
            return Err((StatusCode::SERVICE_UNAVAILABLE, "Request failed!".to_string()));
        }
    };

    if !response.status().is_success() {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    };

    let user = match response.json::<User>().await {
        Ok(user) => user,
        Err(_) => {
            return Err((StatusCode::SERVICE_UNAVAILABLE, "Failed to parse user".to_string()));
        }
    };

    Ok(user)
}
