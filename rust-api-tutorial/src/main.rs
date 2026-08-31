use axum::{Json, Router, routing::get,routing::post};
use serde::{Serialize,Deserialize};
use axum::http::StatusCode;
use axum::extract::{Path, State};
use std::collections::HashMap;
use std::sync::{Arc,Mutex};

type Db=Arc<Mutex<HashMap<u32,User>>>;

#[derive(Serialize)]
struct Greeting {
    message: String,
    status: u16,
}

#[derive(Serialize,Clone)]
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
    let db:Db=Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", get(root))
        .route("/greet", get(greet))
        .route("/user", post(create_user))
        .route("/user/{id}", get(get_user_by_id)).with_state(db);
    

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



async fn create_user(State(db):State<Db>,Json(payload): Json<CreateUser>) -> (StatusCode,Json<User>) {
   let mut db=db.lock().unwrap();
   let id=db.len() as u32 +1;
   let user=User{
    name:payload.name,
    age:payload.age,
    id:id
   };
   db.insert(id, user.clone());
   (StatusCode::CREATED,Json(user))

}

async fn get_user_by_id(State(db): State<Db>,Path(id): Path<u32>)->Result<Json<User>,StatusCode>{
    let db=db.lock().unwrap();
    match db.get(&id){
        Some(user)=> Ok(Json(user.clone())),
        None=>Err(StatusCode::NOT_FOUND),
    }
}


