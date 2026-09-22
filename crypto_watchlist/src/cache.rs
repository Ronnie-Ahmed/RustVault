use crate::errors::AppError;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

pub async fn cache_set<T: Serialize>(
    con: &mut redis::aio::MultiplexedConnection,
    key: &str,
    value: &T,
    ttl_seconds: u64,
) -> Result<(), AppError> {
    let json = serde_json::to_string(value).map_err(|e| AppError::Internal(e.to_string()))?;
    let _: () = con
        .set_ex(key, json, ttl_seconds)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(())
}

pub async fn cache_get<T: for<'a> Deserialize<'a>>(
    con: &mut redis::aio::MultiplexedConnection,
    key: &str,
) -> Result<Option<T>, AppError> {
    let result: Option<String> = con
        .get(key)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    match result {
        Some(json) => {
            let data =
                serde_json::from_str(&json).map_err(|e| AppError::Internal(e.to_string()))?;
            Ok(Some(data))
        }
        None => Ok(None),
    }
}
