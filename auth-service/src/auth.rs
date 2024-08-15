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
    User,
    RefreshTokenRequest,
    RefreshTokenResponse,
    LogoutRequest
};
use crate::AppState;

pub async fn register (
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RegisterDto>
) -> StatusCode {
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
 
    let access_token = utils::generate_access_token(
        &user.id.to_string(),
        &app_state.env.jwt_secret,
        &app_state.env.jwt_access_exp_time_sec
    ).map_err(|e| (StatusCode::SERVICE_UNAVAILABLE, e))?;

    let refresh_token = utils::generate_refresh_token(
        &user.id.to_string(),
        &app_state.env.jwt_secret,
        &app_state.env.jwt_refresh_exp_time_sec
    ).map_err(|e| (StatusCode::SERVICE_UNAVAILABLE, e))?;

    let access_token_key = format!("{}_access_token", user.id);
    let refresh_token_key = format!("{}_refresh_token", user.id);
    
    let access_exp: usize = match app_state.env.jwt_access_exp_time_sec.parse() {
        Ok(exp) => exp,
        Err(_) => return Err((StatusCode::SERVICE_UNAVAILABLE, "Invalid access token expiration time".to_string())),
    };

    let refresh_exp: usize = match app_state.env.jwt_refresh_exp_time_sec.parse() {
        Ok(exp) => exp,
        Err(_) => return Err((StatusCode::SERVICE_UNAVAILABLE, "Invalid refresh token expiration time".to_string())),
    };

    if let Err(_) = tokio::try_join!(
        app_state.redis_store.set(&access_token_key, &access_token, access_exp),
        app_state.redis_store.set(&refresh_token_key, &refresh_token, refresh_exp)
    ) {
        return Err((StatusCode::SERVICE_UNAVAILABLE, "Failed to store tokens in Redis".to_string()));
    }

    let response_json = LoginResponse {
        access_token: access_token.to_string(),
        refresh_token: refresh_token.to_string()
    };

    Ok((StatusCode::OK, Json(response_json)))
}

pub async fn logout(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LogoutRequest>
) -> Result<StatusCode, (StatusCode, String)> {
    let token_data = utils::decode_token(&payload.access_token, &app_state.env.jwt_secret)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid access token".to_string()))?;

    let user_id = token_data.claims.sub;

    let access_token_key = format!("{}_access_token", user_id);
    let refresh_token_key = format!("{}_refresh_token", user_id);

    let remove_access = app_state.redis_store.delete(&access_token_key);
    let remove_refresh = app_state.redis_store.delete(&refresh_token_key);

    if let Err(_) = tokio::try_join!(remove_access, remove_refresh) {
        return Err((StatusCode::SERVICE_UNAVAILABLE, "Failed to remove tokens from Redis".to_string()));
    }

    Ok(StatusCode::OK)
}

pub async fn refresh_token(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>
) -> Result<(StatusCode, Json<RefreshTokenResponse>), (StatusCode, String)> {
    let token_data = utils::decode_token(&payload.refresh_token, &app_state.env.jwt_secret)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid refresh token".to_string()))?;
    
    let user_id = token_data.claims.sub;

    let redis_key = format!("{}_refresh_token", user_id);

    let stored_token = app_state.redis_store.get(&redis_key)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid refresh token".to_string()))?;

    if stored_token != payload.refresh_token {
        return Err((StatusCode::UNAUTHORIZED, "Refresh token mismatch".to_string()));
    }

    let new_access_token = utils::generate_access_token(
        &user_id,
        &app_state.env.jwt_secret,
        &app_state.env.jwt_access_exp_time_sec
    ).map_err(|_| (StatusCode::SERVICE_UNAVAILABLE, "Failed to generate access token".to_string()))?;

    let new_refresh_token = utils::generate_refresh_token(
        &user_id,
        &app_state.env.jwt_secret,
        &app_state.env.jwt_refresh_exp_time_sec
    ).map_err(|_| (StatusCode::SERVICE_UNAVAILABLE, "Failed to generate refresh token".to_string()))?;

    let access_exp: usize = app_state.env.jwt_access_exp_time_sec
        .parse()
        .map_err(|_| (StatusCode::SERVICE_UNAVAILABLE, "Invalid access token expiration time".to_string()))?;

    let refresh_exp: usize = app_state.env.jwt_refresh_exp_time_sec
        .parse()
        .map_err(|_| (StatusCode::SERVICE_UNAVAILABLE, "Invalid refresh token expiration time".to_string()))?;

    let access_token_key = format!("{}_access_token", user_id);
    let refresh_token_key = format!("{}_refresh_token", user_id);

    let store_access = app_state.redis_store.set(&access_token_key, &new_access_token, access_exp);
    let store_refresh = app_state.redis_store.set(&refresh_token_key, &new_refresh_token, refresh_exp);

    if let Err(_) = tokio::try_join!(store_access, store_refresh) {
        return Err((StatusCode::SERVICE_UNAVAILABLE, "Failed to store tokens in Redis".to_string()));
    }

    let response_json = RefreshTokenResponse {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
    };

    Ok((StatusCode::OK, Json(response_json)))
}

async fn create_user(
    State(app_state): State<Arc<AppState>>,
    email: &str,
    username: &str,
    password: &str
) -> StatusCode {
    let http_client = &app_state.http_client;
    let create_user_url = format!("{}/users", &app_state.env.user_service_url);

    match http_client         
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
    let http_client = &app_state.http_client;
    let base_url = &app_state.env.user_service_url;
    let get_user_url = format!("{}/users?email={}", base_url, email);
   
    let response = match http_client.get(&get_user_url).send().await {
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

