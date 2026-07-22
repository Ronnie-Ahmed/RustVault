mod db;
mod error;
mod protocol;

use db::Db;
use protocol::{parse_command, Command};

use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, warn};

const ADDR: &str = "127.0.0.1:6380";


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let db = Db::new();
    
    db.start_background_cleaner(Duration::from_secs(5));

    let listener = TcpListener::bind(ADDR).await?;
    info!("RustVault Started -> {ADDR}");

    loop {
        let (socket, addr) = listener.accept().await?;
        let db = db.clone(); 

        tokio::spawn(async move {
            info!("Accepted A new Connection At: {addr}");
            if let Err(e) = handle_connection(socket, db).await {
                warn!("Connection {addr} found Error: {e}");
            } else {
                info!("Connection {addr} Closed");
            }
        });
    }
}


async fn handle_connection(socket: TcpStream, db: Db) -> anyhow::Result<()> {

    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    writer.write_all(b"RustVault ready. Try: PING, SET k v, GET k, SET k v EX 10, DEL k\n").await?;

    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        let response = execute_line(&line, &db).await;

        writer.write_all(response.as_bytes()).await?;
        writer.write_all(b"\n").await?;
    }

    Ok(())
}


async fn execute_line(line: &str, db: &Db) -> String {
    match parse_command(line) {
        Ok(Command::Ping) => "PONG".to_string(),

        Ok(Command::Get { key }) => db.get(&key).await.unwrap_or_else(|| "(nil)".to_string()),

        Ok(Command::Set { key, value, ttl }) => {
            db.set(key, value, ttl).await;
            "OK".to_string()
        }

        Ok(Command::Del { key }) => {
            if db.delete(&key).await {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }

        Err(e) => {
            error!("Command Parse Error: {e}");
            "Found Error".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn full_command_flow() {
        let db = Db::new();

        assert_eq!(execute_line("PING", &db).await, "PONG");
        assert_eq!(execute_line("SET name rustvault", &db).await, "OK");
        assert_eq!(execute_line("GET name", &db).await, "rustvault");
        assert_eq!(execute_line("DEL name", &db).await, "1");
        assert_eq!(execute_line("GET name", &db).await, "(nil)");
        assert!(execute_line("NOPE", &db).await.starts_with("ERR"));
    }

    #[tokio::test]
    async fn set_with_ttl_expires() {
        let db = Db::new();
        assert_eq!(execute_line("SET k v EX 1", &db).await, "OK");
        assert_eq!(execute_line("GET k", &db).await, "v");
    }
}
