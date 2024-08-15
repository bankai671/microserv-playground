use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct RegisterDto {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginDto {
    pub email: String,
    pub username: String,
    pub password: String
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String
}

#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: usize
}

#[derive(Serialize, Debug)]
pub struct CreateUserRequestDto {
    pub email: String,
    pub username: String,
    pub password: String
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub access_token: String,
}

