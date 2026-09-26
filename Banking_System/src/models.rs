use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserWithHash {
    pub id: i32,
    pub password_hash: String,
}

// #[derive(Deserialize, sqlx::FromRow)]
// pub struct RegisterRequest {
//     pub username: String,
//     pub password: String,
// }

#[derive(Deserialize, sqlx::FromRow)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Nid {
    pub id_no: String,
    pub name: String,
    pub age: i32,
    pub addr: String,
    pub father_name: String,
    pub mother_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateNid {
    pub name: String,
    pub age: i32,
    pub addr: String,
    pub father_name: String,
    pub mother_name: String,
}
#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub id_no: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Userbalance {
    pub user_id: String,
    pub balance: i64,
}

// BD-353CE724-209213

// curl -X POST http://localhost:3006/register_user -H "Content-Type: application/json" -d '{"username": "testuser", "password": "testpass123", "id_no": "BD-353CE724-209213"}'

// eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjEsImV4cCI6MTc5MDQ3MzE0Nn0.8wpyvlLUZjXSMLN2X9Cjel7HlJgw38uEVldwo3xkee4
