use crate::errors::AppError;
use std::collections::HashMap;

pub async fn fetch_prices(coind_ids: &[String]) -> Result<HashMap<String, f64>, AppError> {
    if coind_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let ids_param = coind_ids.join(",");
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        ids_param
    );

    let client = reqwest::Client::builder()
        .user_agent("crypto_watchlist/0.1.0")
        .build()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::UpstreamError(e.to_string()))?;
    if !response.status().is_success() {
        return Err(AppError::UpstreamError(format!(
            "CoinGecko returned status {}",
            response.status()
        )));
    }

    let raw: HashMap<String, HashMap<String, f64>> = response
        .json()
        .await
        .map_err(|e| AppError::UpstreamError(e.to_string()))?;

    let prices = raw
        .into_iter()
        .filter_map(|(coin_id, currencies)| currencies.get("usd").map(|&price| (coin_id, price)))
        .collect();
    Ok(prices)
}
