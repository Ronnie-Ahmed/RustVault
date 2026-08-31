use axum::extract::{Path, State};
use axum::http::{StatusCode, status};
use axum::{Json, Router, routing::get, routing::post,routing::put,routing::delete};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::format;
use axum::response::{IntoResponse,Response};
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<u32, User>>>;

pub enum ApiError{
    NotFound(String),
}

impl IntoResponse for ApiError{
    fn into_response(self) -> Response {
        let (status,message)=match self{
            ApiError::NotFound(msg)=>(StatusCode::NOT_FOUND,msg),
        };
        let body=Json(serde_json::json!({"error":message}));
        (status,body).into_response()
    }
}

#[derive(Serialize)]
struct Greeting {
    message: String,
    status: u16,
}

#[derive(Serialize, Clone)]
pub struct User {
    name: String,
    age: i32,
    id: u32,
}

#[derive(Deserialize)]
pub struct CreateUser {
    name: String,
    age: i32,
}

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", get(root))
        .route("/greet", get(greet))
        .route("/user", get(list_users).post(create_user))
        .route("/user/{id}", get(get_user_by_id).put(update_user).delete(delete_user))
        // .route("/list_users", get(list_users))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello , Rust API!"
}

async fn greet() -> Json<Greeting> {
    Json(Greeting {
        message: "Hello There".to_string(),
        status: 200,
    })
}

async fn update_user(
    State(db): State<Db>,
    Path(id): Path<u32>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, ApiError> {
    let mut db = db.lock().unwrap();
    match db.get_mut(&id) {
        Some(user) => {
            user.name = payload.name;
            user.age = payload.age;
            Ok(Json(user.clone()))
        }
        None => Err(ApiError::NotFound(format!("User Not found {}",id))),
    }
}

async fn delete_user(State(db): State<Db>,Path(id): Path<u32>)->ApiError{
    let mut db=db.lock().unwrap();
    match db.remove(&id){
        Some(_)=>ApiError::NotFound(format!("Successfully deleted {}",id)),
        None => ApiError::NotFound(format!("Not found {}",id))
    }
}

async fn create_user(
    State(db): State<Db>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let mut db = db.lock().unwrap();
    let id = db.len() as u32 + 1;
    let user = User {
        name: payload.name,
        age: payload.age,
        id: id,
    };
    db.insert(id, user.clone());
    (StatusCode::CREATED, Json(user))
}

async fn get_user_by_id(
    State(db): State<Db>,
    Path(id): Path<u32>,
) -> Result<Json<User>, ApiError> {
    let db = db.lock().unwrap();
    match db.get(&id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(ApiError::NotFound(format!("user {} Not found",id))),
    }
}

async fn list_users(State(db): State<Db>) -> Json<Vec<User>> {
    let db = db.lock().unwrap();
    let users: Vec<User> = db.values().cloned().collect();
    Json(users)
}
