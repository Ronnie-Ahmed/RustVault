mod auth;
mod errors;
mod models;

use auth::{AuthUser, create_jwt, hash_password, verify_password};
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use errors::AppError;
use models::{LoginRequest, LoginResponse, RegisterRequest, User, UserWithHash,WatchlistItem,AddWatchlisRequest};
use tracing_subscriber::EnvFilter;
use axum::routing::{get,delete};
use axum::extract::Path;

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
    jwt_secret: String,
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<StatusCode, AppError> {
    let password_hash = hash_password(&payload.password)?;

    sqlx::query("INSERT INTO users (username,password_hash) VALUES ($1,$2)")
        .bind(payload.username)
        .bind(password_hash)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    tracing::info!("New user Registered");
    Ok(StatusCode::CREATED)
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user =
        sqlx::query_as::<_, UserWithHash>("SELECT id,password_hash FROM users WHERE username=$1")
            .bind(&payload.username)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
            .ok_or(AppError::Unauthorized(
                "Invalid username or password".to_string(),
            ))?;

    let valid = verify_password(&payload.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized(
            "invalid username or password".to_string(),
        ));
    }
    let token = create_jwt(user.id, &state.jwt_secret)?;
    tracing::info!(user_id = user.id, "user logged in");
    Ok(Json(LoginResponse { token }))
}

async fn add_to_watchlist(
    State(state): State<AppState>,
    Json(payload): Json<AddWatchlisRequest>,
    auth:AuthUser
)->Result<(StatusCode,Json<WatchlistItem>),AppError>{
    let item=sqlx::query_as::<_,WatchlistItem>(
        "INSERT INTO watchlist_items (user_id,coin_id) VALUES ($1,$2) RETURNING  id,coin_id",
    ).bind(auth.user_id).bind(&payload.coin_id).fetch_one(&state.db).await.map_err(|e| {
        if e.to_string().contains("Unique constraint"){
            AppError::BadRequest("Coin already in watchlist".to_string())
        }else {
            AppError::Internal(e.to_string())
        }
    })?;

    tracing::info!(user_id=auth.user_id,coin_id=%payload.coin_id,"Added to watchlist");
    Ok((StatusCode::CREATED,Json(item)))
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("crypto_watchlist=debug".parse().unwrap()),
        )
        .init();

    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to database");
    tracing::info!("connected to database");

    let state = AppState {
        db: pool,
        jwt_secret,
    };
    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();
    tracing::info!("server listening on http://0.0.0.0:3005");

    axum::serve(listener, app).await.unwrap();

    tracing::info!("Starting Crypto_watchlist server");
}
