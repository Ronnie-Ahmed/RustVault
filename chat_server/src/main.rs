use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};

use axum::extract::State;
use tokio::sync::broadcast;

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<String>,
    db:sqlx::PgPool
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {

    let history=sqlx::query_scalar::<_,String>(
        "SELECT content FROM messages ORDER BY id ASC LIMIT 50"
    ).fetch_all(&state.db)
    .await
    .unwrap_or_default();
    for msg in history{
        if socket.send(Message::Text(msg.into())).await.is_err(){
            return;
        }
    }
    let mut rx = state.tx.subscribe();

    loop {
        tokio::select! {
            // Message arrives from THIS client
            Some(Ok(msg)) = 
                if let Message::Text(text) = msg {
                    let _ = state.tx.send(text.to_string());
                    let db=state.db.clone();
                    let content=text.to_string();
                    tokio::spawn(async move{
                        let _ =sqlx::query("INSERT INTO messages (content) VALUES ($1)")
                            .bind(content)
                            .execute(&db)
                            .await;
                    });
                }
            }
            // Message arrives from the broadcast channel (sent by ANY client)
            Ok(text) = rx.recv() => {
                if socket.send(Message::Text(text.into())).await.is_err() {
                    break;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url=std::env::var("DATABASE_URL").expect("DATABASE URL must be set");
    let pool=sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to database");
    let (tx, _rx) = broadcast::channel(100);
    let state = AppState { tx ,db:pool };
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Websocket server running on ws://0.0.0.0:3001/ws");
    axum::serve(listener, app).await.unwrap()
}
