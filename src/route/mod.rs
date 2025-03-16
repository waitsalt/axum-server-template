mod user;

use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    trace,
};

pub fn init() -> Router {
    let user_router = user::init();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_methods(Any);
    Router::new()
        .nest("/api/user", user_router)
        .layer(trace::TraceLayer::new_for_http())
        .layer(cors)
}
