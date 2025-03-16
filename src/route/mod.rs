use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    trace,
};

pub fn init() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_methods(Any);
    Router::new()
        .layer(trace::TraceLayer::new_for_http())
        .layer(cors)
}
