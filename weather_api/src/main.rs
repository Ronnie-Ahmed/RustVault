use serde::Deserialize;
use axum::{Router,routing::get,extract::Query,Json,http::StatusCode,response::IntoResponse,response::Response};
use serde::Serialize;
#[derive(Deserialize)]
struct GeocodingResponse {
    results: Option<Vec<GeocodingResult>>,
}

#[derive(Deserialize)]
pub struct GeocodingResult {
    name: String,
    latitude: f64,
    longitude: f64,
    country: String,
}

#[derive(Deserialize, Serialize)]
struct ForcastResponse {
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Serialize,Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
    weathercode: i32,
}

#[derive(Deserialize)]
struct WeatherParams{
    city:String,
}

enum ApiError{
    NotFound(String),
    UpstreamError(String),
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::UpstreamError(msg) => (StatusCode::BAD_GATEWAY, msg),
        };
        Json(serde_json::json!({ "error": message })).into_response()
    }
}


async fn weather_handler(Query(params): Query<WeatherParams>)->Result<Json<CurrentWeather>,ApiError>{
    let weather=get_weather_for_city(&params.city)
        .await
        .map_err(|e|{
            if e=="City not found"{
                ApiError::NotFound(e)
            }else{
                ApiError::UpstreamError(e)
            }
        })?;
        Ok(Json(weather))
}

#[tokio::main]
async fn main() {
    // let response = reqwest::get("https://geocoding-api.open-meteo.com/v1/search?name=London")
    //     .await
    //     .unwrap()
    //     .text()
    //     .await
    //     .unwrap();
    // println!("{}", response);

    // match get_weather_for_city("London").await{
    //     Ok(weather)=> println!("{:?}",weather),
    //     Err(e)=>println!("Error {:?}",e),
    // }

    let app=Router::new().route("/weather", get(weather_handler));
    let listener=tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_weather_for_city(city: &str) -> Result<CurrentWeather, String> {
    let client = reqwest::Client::new();
    let geo_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}",
        city
    );
    let geo_response: GeocodingResponse = client
        .get(&geo_url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;
    let results = geo_response.results.ok_or("City not found".to_string())?;
    let first = results
        .into_iter()
        .next()
        .ok_or("City not found".to_string())?;
    let forecast_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        first.latitude, first.longitude
    );
    let forecast: ForcastResponse = client
        .get(&forecast_url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;
    Ok(forecast.current_weather)
}
