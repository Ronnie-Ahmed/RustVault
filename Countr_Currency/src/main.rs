use axum::{
    Json, Router,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
#[derive(Debug, Deserialize)]
pub struct CountryCode {
    country_name: String,
    currency_code: String,
    currency_symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CountryCurrency {
    amount: f64,
    base: String,
    date: String,
    rates: HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
pub struct CountryRates {
    country_currency: CountryCurrency,
}

#[derive(Deserialize)]
struct Countryparams {
    country: String,
}

async fn country_handler(
    Query(params): Query<Countryparams>,
) -> Result<Json<CountryCurrency>, ApiError> {
    let country = get_currency_rate(&params.country).await.map_err(|e| {
        if e == "Could not find country code" {
            ApiError::NotFound(e)
        } else {
            ApiError::UpstreamError(e)
        }
    })?;
    Ok(Json(country))
}

enum ApiError {
    NotFound(String),
    UpstreamError(String),
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::UpstreamError(msg) => (StatusCode::BAD_GATEWAY, msg),
        };
        let body = Json(serde_json::json!({"error":message}));
        (status, body).into_response()
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // let api_key = std::env::var("API_KEY").expect("API KEY must be set");
    // let client=reqwest::Client::new();
    // let response=client.get("https://api.restcountries.com/countries/v5?q=canada").header("Authorization", format!("Bearer {}",&api_key)).send().await.unwrap();
    // let text_response=response.text().await.unwrap();

    // println!("{:?}",text_response);
    // match get_currency_rate("japan").await {
    //     Ok(response) => println!("{:?}", response),
    //     Err(e) => println!("Error {}", e.to_string()),
    // }

    let app = Router::new().route("/country", get(country_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    println!("Server Started AT : http://0.0.0.0:3003");
    axum::serve(listener, app).await.unwrap();
}

async fn get_currency_rate(country: &str) -> Result<CountryCurrency, String> {
    let client = reqwest::Client::new();
    let api_key = std::env::var("API_KEY").expect("API KEY MUST BE SET");
    let header_url = format!("Bearer {}", &api_key);
    let country_url = format!("https://api.restcountries.com/countries/v5?q={}", country);
    let country_response: Value = client
        .get(&country_url)
        .header("Authorization", &header_url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;
    let currency_code = country_response["data"]["objects"][0]["currencies"][0]["code"]
        .as_str()
        .ok_or("Could not find country code".to_string())?
        .to_string();

    let currency_symbol = country_response["data"]["objects"][0]["currencies"][0]["symbol"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let country_name = country_response["data"]["objects"][0]["names"]["official"]
        .as_str()
        .ok_or("Could not find name".to_string())?
        .to_string();
    // Ok(CountryCode { country_name, currency_code, currency_symbol })
    let currency_url = format!(
        "https://api.frankfurter.dev/v1/latest?base={}&symbols=USD",
        currency_code
    );
    let currency_response: CountryCurrency = client
        .get(&currency_url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    // let currency=CountryCurrency{
    //     amount:currency_response.amount,
    //     base:currency_response.base,
    //     date:currency_response.date,
    //     rates:currency_response.rates
    // };
    Ok(CountryCurrency {
        amount: currency_response.amount,
        base: currency_response.base,
        date: currency_response.date,
        rates: currency_response.rates,
    })
}
