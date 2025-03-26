use axum::{
    Router,
    routing::{get, post},
};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route("/signin", post(service::user::login))
        .route("/signout", get(service::user::logout))
        .route("/signup", post(service::user::create))
}
