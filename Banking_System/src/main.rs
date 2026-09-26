mod auth;
mod db;
mod errors;
mod models;
use auth::{AuthUser, create_jwt, hash_password, verify_password};
use axum::{Json, Router, extract::State, routing::post};
use models::{CreateNid, Nid};
use reqwest::StatusCode;
use tracing_subscriber::EnvFilter;

use crate::{
    errors::AppError,
    models::{
        CreateUser, DepositRequest, LoginRequest, LoginResponse, User, UserWithHash, Userbalance,
    },
};

#[derive(Debug, Clone)]
pub struct AppState {
    db: sqlx::PgPool,
    jwt_secret: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("banking_system=debug".parse().unwrap()),
        )
        .init();

    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Database url must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT secret must be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to the database");
    let state = AppState {
        db: pool,
        jwt_secret,
    };

    let app = Router::new()
        .route("/register_user", post(register_user))
        .route("/login", post(login))
        .route("/register_nid_card", post(register_nid))
        .route("/create_bank_account", post(create_bank_account))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3006").await.unwrap();
    tracing::info!("server Listening on http://0.0.0.0:3006");
    axum::serve(listener, app).await.unwrap();
    tracing::info!("Starting Crypto_watchlist server");
}

// pub async fn register(
//     State(state): State<AppState>,
//     Json(payload): Json<RegisterRequest>,
// ) -> Result<StatusCode, AppError> {
//     let password_hash = hash_password(&payload.password)?;
//     sqlx::query("INSERT INTO users (username,password_hash) VALUES ($1,$2)")
//         .bind(payload.username)
//         .bind(password_hash)
//         .execute(&state.db)
//         .await
//         .map_err(|e| AppError::Internal(e.to_string()))?;
//     tracing::info!("New user Registered");
//     Ok(StatusCode::CREATED)
// }

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user =
        sqlx::query_as::<_, UserWithHash>("SELECT id,password_hash FROM users WHERE username=$1")
            .bind(&payload.username)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
            .ok_or(AppError::UnAuthorize(
                "Invalid username or password".to_string(),
            ))?;
    let valid = verify_password(&payload.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::UnAuthorize(
            "invalid username or password".to_string(),
        ));
    }
    let token = create_jwt(user.id, &state.jwt_secret)?;
    tracing::info!(user_id = user.id, "user logged in");
    Ok(Json(LoginResponse { token }))
}

pub async fn register_nid(
    State(state): State<AppState>,
    Json(payload): Json<CreateNid>,
) -> Result<Json<Nid>, AppError> {
    let id_no = generate_id(&payload);

    let id_card = sqlx::query_as::<_, Nid>(
        "INSERT INTO nid (id_no, name, age, addr, father_name, mother_name)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id_no, name, age, addr, father_name, mother_name",
    )
    .bind(id_no)
    .bind(payload.name)
    .bind(payload.age)
    .bind(payload.addr)
    .bind(payload.father_name)
    .bind(payload.mother_name)
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(id_card))
}

use rand::Rng;
use sha2::{Digest, Sha256};
pub fn generate_id(nid: &CreateNid) -> String {
    let mut rng = rand::rng();

    let random_number: u32 = rng.random_range(100_000..=999_999);

    let input = format!(
        "{}|{}|{}|{}|{}|{}",
        nid.name, nid.age, nid.addr, nid.father_name, nid.mother_name, random_number
    );

    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());

    let hash = hasher.finalize();

    let hash_part = format!(
        "{:02X}{:02X}{:02X}{:02X}",
        hash[0], hash[1], hash[2], hash[3]
    );

    format!("BD-{}-{}", hash_part, random_number)
}

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let password_hash = hash_password(&payload.password)?;
    let user = sqlx::query_as::<_, User>(
        "
    INSERT INTO users (username,password_hash,nid_no) VALUES ($1,$2,$3) 
    RETURNING id,username
    ",
    )
    .bind(payload.username)
    .bind(password_hash)
    .bind(payload.id_no)
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;
    tracing::info!("New user Registered");
    Ok(Json(user))
}

pub async fn create_bank_account(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Userbalance>, AppError> {
    let user = sqlx::query_as::<_, Userbalance>(
        "INSERT INTO bank_information (user_id) VALUES ($1) RETURNING user_id, balance",
    )
    .bind(auth.user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        if e.to_string().contains("unique constraint") {
            AppError::BadRequest("bank account already exists for this user".to_string())
        } else {
            AppError::Internal(e.to_string())
        }
    })?;
    Ok(Json(user))
}

pub async fn deposit(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(payload): Json<DepositRequest>,
) -> Result<Json<Userbalance>, AppError> {
    if payload.amount <= 0 {
        return Err(AppError::BadRequest(
            "deposit amount must be positive".to_string(),
        ));
    }

    let updated=sqlx::query_as::<_,Userbalance>(
        "UPDATE bank_information SET balance=balance + {$1} WHERE user_id = {$2} RETURNING user_id,balance"
    ).bind(payload.amount)
    .bind(auth.user_id).fetch_optional(&state.db).await.map_err(|e| AppError::NotFound(e.to_string()))?.ok_or(AppError::NotFound("bank account not found".to_string()))?;
    Ok(Json(updated))
}


pub async fn get_balance(
    State(state): State<AppState>,
    auth: AuthUser,
    
) -> Result<Json<Userbalance>, AppError> {
    let balance=sqlx::query_as::<_,Userbalance>(
        "SELECT (user_id,balance) FROM bank_information  WHERE user_id = {$1} RETURNING user_id,balance"
    )
    .bind(auth.user_id).fetch_optional(&state.db).await.map_err(|e| AppError::NotFound(e.to_string()))?.ok_or(AppError::NotFound("bank account not found".to_string()))?;
    Ok(Json(balance))
}
