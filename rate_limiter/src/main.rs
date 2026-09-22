use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
// use axum::body::Body;
// use axum::http::{Request, StatusCode};
// use axum::middleware::from_fn_with_state;
// use axum::response::Response;
// use axum_rate_limiter::settings::Settings;
// use axum_rate_limiter::limiter::{RateLimiterManager, middleware as rate_limiter_middleware};

#[derive(Clone)]
struct RateLimiter {
    requests: Arc<Mutex<HashMap<IpAddr, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

// async fn handler()->impl IntoResponse{
//     "Hello World"
// }

// // struct Server{}

// // impl Server{
// //     async fn run(self)->Result<(),std::io::Error>{
// //         let listener=tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
// //         let settings=Settings::new().expect("Failed to create setting");
// //         let limit_manager=Arc::new(
// //             RateLimiterManager::new(settings.rate_limiter_settings)
// //                 .expect("Failed To create rate Limit")
// //         );

// //         let app=Router::new().route("/home", get(handler)).layer(from_fn_with_state(limit_manager, rate_limit_middleware));
// //          println!("Server running on http://0.0.0.0:3003");
// //          axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await
// //     }
// // }

// // async fn rate_limit_middleware(
// //     State(limiter): State<RateLimiter>,
// //     ConnectInfo(addr): ConnectInfo<SocketAddr>,
// //     request: Request,
// //     next: Next,
// // ) -> Result<Response, StatusCode> {
// //     let ip = addr.ip();
// //     let now = Instant::now();

// //     let mut requests = limiter.requests.lock().unwrap();
// //     let timestamps = requests.entry(ip).or_insert_with(Vec::new);

// //     timestamps.retain(|&time| now.duration_since(time) < limiter.window);

// //     if timestamps.len() >= limiter.max_requests {
// //         return Err(StatusCode::TOO_MANY_REQUESTS);
// //     }

// //     timestamps.push(now);
// //     drop(requests);

// //     Ok(next.run(request).await)
// // }

// #[tokio::main]
// async fn main() {
//     let server=Server{};
//     if let Err(e) = server.run().await {
//         eprintln!("Server Error: {}",e)

//     };
//     // let limiter = RateLimiter {
//     //     requests: Arc::new(Mutex::new(HashMap::new())),
//     //     max_requests: 5,
//     //     window: Duration::from_secs(10),
//     // };

//     // let app = Router::new()
//     //     .route("/home", get(test))
//     //     .layer(from_fn_with_state(limiter, rate_limit_middleware));

//     // let listener = tokio::net::TcpListener::bind("0.0.0.0:3003")
//     //     .await
//     //     .unwrap();

//     // println!("Server running at: http://0.0.0.0:3003");

//     // axum::serve(
//     //     listener,
//     //     app.into_make_service_with_connect_info::<SocketAddr>(),
//     // )
//     // .await
//     // .unwrap();
// }

// // async fn test() -> String {
// //     "Hello World".to_string()
// // }

use axum::{Router, middleware, routing::get};
use axum::{
    extract::Request,
    extract::{ConnectInfo, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::net::SocketAddr;

async fn rate_limit_middleware(
    State(limiter): State<RateLimiter>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // temporarily hardcode instead of using ConnectInfo
    let now = Instant::now();
    let mut requests = limiter.requests.lock().unwrap();
    let timestamps = requests
        .entry("127.0.0.1".parse().unwrap())
        .or_insert_with(Vec::new);

    timestamps.retain(|&time| now.duration_since(time) < limiter.window);

    if timestamps.len() >= limiter.max_requests {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    timestamps.push(now);
    drop(requests);

    Ok(next.run(request).await)
}
async fn hello() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let limiter = RateLimiter {
        requests: Arc::new(Mutex::new(HashMap::new())),
        max_requests: 5,
        window: Duration::from_secs(10),
    };

    // let app = Router::new()
    //     .route("/hello", get(hello))
    //     .layer(middleware::from_fn_with_state(limiter.clone(), rate_limit_middleware))
    //     .with_state(limiter);
    let app = Router::new().route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3004").await.unwrap();
    println!("Rate limiter running on http://0.0.0.0:3004");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}
