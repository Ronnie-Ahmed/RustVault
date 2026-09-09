use axum::extract::Query;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{post, put},
};
use serde::{Deserialize, Serialize};
pub enum ApiError {
    NotFound(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        };
        let body = Json(serde_json::json!({"error":message}));
        (status, body).into_response()
    }
}

// type Db = Arc<Mutex<HashMap<u32, Task>>>;
type Db = sqlx::PgPool;

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct Task {
    id: i32,
    title: String,
    done: bool,
    user_id:i32,
}

#[derive(Deserialize)]
pub struct CreateTask {
    title: String,
    user_id:i32,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    title: String,
    done: bool,
}

#[derive(Deserialize)]
pub struct TaskFilter {
    done: Option<bool>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");
    println!("Connected to database successfully");
    // let db = Arc::new(Mutex::new(HashMap::new()));
    let db = pool;
    let app = Router::new()
        .route("/task", post(create_task).get(list_tasks))
        .route(
            "/task/{id}",
            put(update_task).get(get_task_by_id).delete(delete_task),
        )
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listeting on port http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_task_by_id(State(db): State<Db>, Path(id): Path<i32>) -> Result<Json<Task>, ApiError> {
    let task = sqlx::query_as::<_, Task>("SELECT id,title,done,user_id FROM tasks WHERE id=$1")
        .bind(id)
        .fetch_optional(&db)
        .await
        .unwrap();

    // let db = db.lock().unwrap();
    match task {
        Some(task) => Ok(Json(task)),
        None => Err(ApiError::NotFound(format!("task not found {}", id))),
    }
}

async fn delete_task(State(db): State<Db>, Path(id): Path<i32>) -> Result<StatusCode, ApiError> {
    // let mut db = db.lock().unwrap();
    let result = sqlx::query("DELETE FROM tasks WHERE id=$1")
        .bind(id)
        .execute(&db)
        .await
        .unwrap();
    // match db.remove(&id) {
    //     Some(_) => Ok(StatusCode::NO_CONTENT),
    //     None => Err(ApiError::NotFound(format!("task not found {}" , id))),
    // }
    if result.rows_affected() == 0 {
        Err(ApiError::NotFound(format!("task not found {}", id)))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

async fn update_task(
    State(db): State<Db>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, ApiError> {
    // let mut db = db.lock().unwrap();
    let result = sqlx::query_as::<_, Task>(
        "UPDATE tasks SET title=$1 , done=$2 WHERE id=$3 RETURNING id,title,done,user_id",
    )
    .bind(payload.title)
    .bind(payload.done)
    .bind(id)
    .fetch_optional(&db)
    .await
    .unwrap();
    match result {
        Some(task) => Ok(Json(task)),
        None => Err(ApiError::NotFound(format!("Task Not found {}", id))),
    }

    // match db.get_mut(&id) {
    //     Some(task) => {
    //         task.title = payload.title;
    //         task.done = payload.done;
    //         Ok(Json(task.clone()))
    //     }
    //     None => Err(ApiError::NotFound(format!("Task Not found {}",id))),
    // }
}

async fn create_task(
    State(db): State<Db>,
    Json(payload): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    // let mut db = db.lock().unwrap();
    // let id = db.len() as u32 + 1;
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (title,done,user_id) VALUES ($1,false,$2) RETURNING id, title,done,user_id",
    )
    .bind(payload.title)
    .bind(payload.user_id)
    .fetch_one(&db)
    .await
    .unwrap();

    // let task = Task {
    //     id: id,
    //     title: payload.title,
    //     done: false,
    // };
    // db.insert(id, task.clone());
    (StatusCode::CREATED, Json(task))
}

async fn list_tasks(State(db): State<Db>, Query(filter): Query<TaskFilter>) -> Json<Vec<Task>> {
    // let db = db.lock().unwrap();
    // let tasks = db.values().filter(|task| {
    //     match filter.done{
    //         Some(done)=>task.done==done,
    //         None=>true
    //     }
    // }).cloned().collect();
    // Json(tasks)
    let tasks = match filter.done {
        Some(done) => sqlx::query_as::<_, Task>("SELECT id,title,done,user_id FROM tasks WHERE done=$1")
            .bind(done)
            .fetch_all(&db)
            .await
            .unwrap(),
        None => sqlx::query_as::<_, Task>("SELECT id,title,done,user_id FROM tasks")
            .fetch_all(&db)
            .await
            .unwrap(),
    };
    Json(tasks)
}
