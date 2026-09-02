use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{
    Json, Router,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::response::{IntoResponse,Response};
use axum::extract::Query;
pub enum ApiError{
    NotFound(String)
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

type Db = Arc<Mutex<HashMap<u32, Task>>>;

#[derive(Serialize, Clone)]
pub struct Task {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
pub struct CreateTask {
    title: String,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    title: String,
    done:bool
}



#[derive(Deserialize)]
pub struct TaskFilter{
    done:Option<bool>
}

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/task", post(create_user).get(list_tasks))
        .route(
            "/task/{id}",
            put(update_task).get(get_task_by_id).delete(delete_task),
        )
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listeting on port http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}



async fn get_task_by_id(
    State(db): State<Db>,
    Path(id): Path<u32>,
) -> Result<Json<Task>, ApiError> {
    let db = db.lock().unwrap();
    match db.get(&id) {
        Some(task) => Ok(Json(task.clone())),
        None => Err(ApiError::NotFound(format!("task not found {}" , id))),
    }
}

async fn delete_task(State(db): State<Db>, Path(id): Path<u32>) -> Result<StatusCode,ApiError> {
    let mut db = db.lock().unwrap();
    match db.remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(ApiError::NotFound(format!("task not found {}" , id))),
    }
}

async fn update_task(
    State(db): State<Db>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, ApiError> {
    let mut db = db.lock().unwrap();

    match db.get_mut(&id) {
        Some(task) => {
            task.title = payload.title;
            task.done = payload.done;
            Ok(Json(task.clone()))
        }
        None => Err(ApiError::NotFound(format!("Task Not found {}",id))),
    }
}

async fn create_user(
    State(db): State<Db>,
    Json(payload): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    let mut db = db.lock().unwrap();
    let id = db.len() as u32 + 1;

    let task = Task {
        id: id,
        title: payload.title,
        done: false,
    };
    db.insert(id, task.clone());
    (StatusCode::CREATED, Json(task))
}

async fn list_tasks(State(db): State<Db>,Query(filter): Query<TaskFilter>) -> Json<Vec<Task>> {
    let db = db.lock().unwrap();
    let tasks = db.values().filter(|task| {
        match filter.done{
            Some(done)=>task.done==done,
            None=>true
        }
    }).cloned().collect();
    Json(tasks)
}
