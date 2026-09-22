use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::post;
use redis::AsyncCommands;

use axum::{Router, routing::get};

use serde::{Deserialize, Serialize};
//#[derive(Debug, Deserialize,Serialize, Clone,sqlx::FromRow)]
#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Product {
    id: i32,
    content: String,
    is_sold: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProduct {
    content: String,
    is_sold: bool,
}

type Db = sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    db: Db,
    redis: redis::aio::MultiplexedConnection,
}

#[tokio::main]
async fn main() {
    let redis_client =
        redis::Client::open("redis://127.0.0.1:6380/").expect("Invalid Redis Connection");
    let redis_conn = redis_client
        .get_multiplexed_async_connection()
        .await
        .expect("Failed to get multiplexed connection");

    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DataBase url must be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("failed to connect to database");
    println!("Connected to database Successfully");

    let state = AppState {
        db: pool,
        redis: redis_conn,
    };

    let app = Router::new()
        .route("/home", get(test))
        .route("/product", post(create_product).get(list_product))
        .route(
            "/product/{id}",
            get(get_product_by_id)
                .put(update_product)
                .delete(delete_rpoduct),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    println!("Server Listening on port : http://0.0.0.0:3003");
    axum::serve(listener, app).await.unwrap();
}

async fn test() -> Result<String, String> {
    Ok(format!("Ronnie"))
}

//-------------------------------------------------Cache

async fn cache_set<T: Serialize>(
    con: &mut redis::aio::MultiplexedConnection,
    key: &str,
    value: &T,
    ttl_seconds: u64,
) -> Result<(), redis::RedisError> {
    let json = serde_json::to_string(value).map_err(|e| {
        redis::RedisError::from((
            redis::ErrorKind::TypeError,
            "JSON serialization failed",
            e.to_string(),
        ))
    })?;

    let _: () = con.set_ex(key, json, ttl_seconds).await?;

    Ok(())
}

async fn cache_get<T: for<'a> Deserialize<'a>>(
    con: &mut redis::aio::MultiplexedConnection,
    key: &str,
) -> Result<Option<T>, redis::RedisError> {
    let result: Option<String> = con.get(key).await?;

    match result {
        Some(json) => {
            let data: T = serde_json::from_str(&json).map_err(|e| {
                redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "JSON deserialization failed",
                    e.to_string(),
                ))
            })?;

            Ok(Some(data))
        }

        None => Ok(None),
    }
}

async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProduct>,
) -> (StatusCode, Json<Product>) {
    let build_product = sqlx::query_as::<_, Product>(
        "
    INSERT INTO product (content,is_sold) VALUES ($1,false) RETURNING  id,content,is_sold",
    )
    .bind(payload.content)
    .fetch_one(&state.db)
    .await
    .unwrap();

    let cache_key = format!("product:{}", build_product.id);
    let mut redis = state.redis.clone();
    if let Err(err) = cache_set(&mut redis, &cache_key, &build_product, 60 * 6).await {
        eprintln!("Redis SET error : {}", err);
    }
    (StatusCode::CREATED, Json(build_product))
}

async fn list_product(State(state): State<AppState>) -> Json<Vec<Product>> {
    let all_product = sqlx::query_as::<_, Product>("SELECT id ,content,is_sold FROM product")
        .fetch_all(&state.db)
        .await
        .unwrap();

    Json(all_product)
}

async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateProduct>,
) -> Result<Json<Product>, StatusCode> {
    // let cache_key=format!("product:{}",id);
    // let mut redis=state.redis.clone();
    // match cache_get::<Product>(&mut redis, &cache_key).await
    let product = sqlx::query_as::<_, Product>(
        "UPDATE product SET content=$1 ,is_sold=$2 WHERE id=$3 RETURNING  id,content,is_sold",
    )
    .bind(payload.content)
    .bind(payload.is_sold)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .unwrap();
    let product = match product {
        Some(product) => product,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Invalidate cache
    let cache_key = format!("product:{}", id);
    let mut redis = state.redis.clone();

    if let Err(error) = redis::cmd("DEL")
        .arg(&cache_key)
        .query_async::<()>(&mut redis)
        .await
    {
        eprintln!("Redis DEL error: {}", error);
    }
    Ok(Json(product))
}

async fn get_product_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Product>, StatusCode> {
    let cache_key = format!("product:{}", id);
    let mut redis = state.redis.clone();

    // --------------------------------------------------
    // 1. Check Redis cache
    // --------------------------------------------------

    match cache_get::<Product>(&mut redis, &cache_key).await {
        Ok(Some(product)) => {
            println!("CACHE HIT: {}", cache_key);
            return Ok(Json(product));
        }
        Ok(None) => {
            println!("CACHE MISS: {}", cache_key);
        }
        Err(error) => {
            eprintln!("Redis GET error: {}", error);
        }
    }
    // --------------------------------------------------
    // 2. Cache miss -> Query PostgreSQL
    // --------------------------------------------------

    let get_product = sqlx::query_as::<_, Product>(
        "
    SELECT id,content,is_sold FROM product WHERE id=$1",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .unwrap();
    let product = match get_product {
        Some(productt) => productt,
        None => return Err(StatusCode::NOT_FOUND),
    };

    if let Err(error) = cache_set(&mut redis, &cache_key, &product, 60 * 6).await {
        eprintln!("Redis SET error : {}", error);
    }

    Ok(Json(product))
}

async fn delete_rpoduct(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM product WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|error| {
            eprintln!("Database error: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Delete Redis cache
    let cache_key = format!("product:{}", id);
    let mut redis = state.redis.clone();

    if let Err(error) = redis::cmd("DEL")
        .arg(&cache_key)
        .query_async::<()>(&mut redis)
        .await
    {
        eprintln!("Redis DEL error: {}", error);
    }

    Ok(StatusCode::NO_CONTENT)
}
