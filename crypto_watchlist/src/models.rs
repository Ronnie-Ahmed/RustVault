use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(sqlx::FromRow)]
pub struct UserWithHash {
    pub id: i32,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Clone, sqlx::FromRow)]
pub struct WatchlistItem {
    pub id: i32,
    pub coin_id: String,
}

#[derive(Deserialize)]
pub struct AddWatchlistRequest {
    pub coin_id: String,
}
