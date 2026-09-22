mod auth;
mod cache;
mod coingecko;
mod errors;
mod models;

use axum::middleware;
use auth::{AuthUser, create_jwt, hash_password, verify_password};
use axum::extract::Path;
use axum::routing::{delete, get};
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use errors::AppError;
use models::{
    AddWatchlistRequest, Alert, CreateAlertRequest, LoginRequest, LoginResponse, RegisterRequest,
     UserWithHash, WatchlistItem,
};
use std::sync::{Arc, Mutex};
use std::net::IpAddr;
use std::time::{Duration, Instant};
use axum::{middleware::Next, extract::{ConnectInfo, Request}, response::Response};

use std::net::SocketAddr;

use std::collections::HashMap;
use tracing_subscriber::EnvFilter;
use axum::response::IntoResponse;


const MAX_REQUESTS: usize = 20;
const WINDOW: Duration = Duration::from_secs(10);


#[derive(Clone, Debug)]
struct AppState {
    db: sqlx::PgPool,
    jwt_secret: String,
    redis: redis::aio::MultiplexedConnection,
     rate_limits: Arc<Mutex<HashMap<IpAddr, Vec<Instant>>>>,
}

async fn rate_limit_middleware(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    let ip = addr.ip();
    let now = Instant::now();

    let mut limits = state.rate_limits.lock().unwrap();
    let timestamps = limits.entry(ip).or_insert_with(Vec::new);

    timestamps.retain(|&t| now.duration_since(t) < WINDOW);

    if timestamps.len() >= MAX_REQUESTS {
        return AppError::BadRequest("rate limit exceeded".to_string()).into_response();
    }

    timestamps.push(now);
    drop(limits);

    next.run(request).await
}

async fn create_alert(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(payload): Json<CreateAlertRequest>,
) -> Result<(StatusCode, Json<Alert>), AppError> {
    if payload.direction != "above" && payload.direction != "below" {
        return Err(AppError::BadRequest(
            "direction must be 'above' or 'below'".to_string(),
        ));
    }

    let alert = sqlx::query_as::<_, Alert>(
        "INSERT INTO alerts (user_id, coin_id, target_price, direction)
         VALUES ($1, $2, $3, $4)
         RETURNING id, coin_id, target_price, direction, triggered",
    )
    .bind(auth.user_id)
    .bind(&payload.coin_id)
    .bind(payload.target_price)
    .bind(&payload.direction)
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    tracing::info!(user_id = auth.user_id, coin_id = %payload.coin_id, "alert created");
    Ok((StatusCode::CREATED, Json(alert)))
}

async fn list_alerts(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<Alert>>, AppError> {
    let alerts = sqlx::query_as::<_, Alert>(
        "SELECT id, coin_id, target_price, direction, triggered FROM alerts WHERE user_id = $1 ORDER BY id"
    )
    .bind(auth.user_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(alerts))
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

async fn get_prices(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<HashMap<String, f64>>, AppError> {
    let items = sqlx::query_as::<_, WatchlistItem>(
        "SELECT id, coin_id FROM watchlist_items WHERE user_id = $1",
    )
    .bind(auth.user_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let coin_ids: Vec<String> = items.into_iter().map(|item| item.coin_id).collect();

    let mut redis = state.redis.clone();
    let cache_key = format!("prices:{}", coin_ids.join(","));

    if let Ok(Some(cached)) = cache::cache_get::<HashMap<String, f64>>(&mut redis, &cache_key).await
    {
        tracing::info!(user_id = auth.user_id, "prices cache hit");
        return Ok(Json(cached));
    }

    tracing::info!(user_id = auth.user_id, "prices cache miss");
    let prices = coingecko::fetch_prices(&coin_ids).await?;

    let _ = cache::cache_set(&mut redis, &cache_key, &prices, 30).await;

    Ok(Json(prices))
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
    auth: AuthUser,
    Json(payload): Json<AddWatchlistRequest>,
) -> Result<(StatusCode, Json<WatchlistItem>), AppError> {
    let item = sqlx::query_as::<_, WatchlistItem>(
        "INSERT INTO watchlist_items (user_id, coin_id) VALUES ($1, $2) RETURNING id, coin_id",
    )
    .bind(auth.user_id)
    .bind(&payload.coin_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        if e.to_string().contains("unique constraint") {
            AppError::BadRequest("coin already in watchlist".to_string())
        } else {
            AppError::Internal(e.to_string())
        }
    })?;

    tracing::info!(user_id = auth.user_id, coin_id = %payload.coin_id, "added to watchlist");
    Ok((StatusCode::CREATED, Json(item)))
}

async fn run_alert_checker(db: sqlx::PgPool) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));

    loop {
        interval.tick().await;
        tracing::info!("running alert check");

        if let Err(e) = check_alerts(&db).await {
            tracing::error!(error = %e, "alert check failed");
        }
    }
}

async fn check_alerts(db: &sqlx::PgPool) -> Result<(), AppError> {
    let alerts = sqlx::query_as::<_, (i32, String, f64, String)>(
        "SELECT id,coin_id,target_price,direction FROM alerts WHERE triggered=false",
    )
    .fetch_all(db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;
    if alerts.is_empty() {
        return Ok(());
    }

    let coin_ids: Vec<String> = alerts
        .iter()
        .map(|(_, coin_id, _, _)| coin_id.clone())
        .collect();
    let coin_ids: Vec<String> = coin_ids
        .into_iter()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    let prices = coingecko::fetch_prices(&coin_ids).await?;

    for (id, coin_id, target_price, direction) in alerts {
        if let Some(&current_price) = prices.get(&coin_id) {
            let should_trigger = match direction.as_str() {
                "above" => current_price >= target_price,
                "below" => current_price <= target_price,
                _ => false,
            };

            if should_trigger {
                sqlx::query("UPDATE alerts SET triggered = true WHERE id = $1")
                    .bind(id)
                    .execute(db)
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?;

                tracing::info!(alert_id = id, coin_id = %coin_id, current_price, target_price, "alert triggered");
            }
        }
    }

    Ok(())
}

async fn list_watchlist(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<WatchlistItem>>, AppError> {
    let items = sqlx::query_as::<_, WatchlistItem>(
        "SELECT id,coin_id FROM watchlist_items WHERE user_id=$1 ORDER BY id",
    )
    .bind(auth.user_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(items))
}

async fn remove_from_watchlist(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM watchlist_items WHERE id =$1 AND user_id=$2")
        .bind(id)
        .bind(auth.user_id)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Watchlist item not found".to_string()));
    }
    tracing::info!(
        user_id = auth.user_id,
        item_id = id,
        "Removed form watchlist"
    );
    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("crypto_watchlist=debug".parse().unwrap()),
        )
        .init();
    let redis_client =
        redis::Client::open("redis://127.0.0.1:6381/").expect("Invalid Redis Connection");
    let redis_conn = redis_client
        .get_multiplexed_async_connection()
        .await
        .expect("Failed to get multiplexed connection");

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
    redis: redis_conn,
    rate_limits: Arc::new(Mutex::new(HashMap::new())),
};

    // let prices = coingecko::fetch_prices(&["bitcoin".to_string(), "ethereum".to_string()]).await;
    // tracing::info!(?prices, "test fetch");
    let db_for_checker = state.db.clone();
    tokio::spawn(run_alert_checker(db_for_checker));
    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/watchlist", post(add_to_watchlist).get(list_watchlist))
        .route("/watchlist/{id}", delete(remove_from_watchlist))
        .route("/prices", get(get_prices))
        .route("/alerts", post(create_alert).get(list_alerts))
         .layer(middleware::from_fn_with_state(state.clone(), rate_limit_middleware))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();
    tracing::info!("server listening on http://0.0.0.0:3005");

    axum::serve(
    listener,
    app.into_make_service_with_connect_info::<SocketAddr>(),
)
.await
.unwrap();
    tracing::info!("Starting Crypto_watchlist server");
}

fn assert_send_sync<T: Send + Sync>() {}

fn _check_appstate() {
    assert_send_sync::<AppState>();
}