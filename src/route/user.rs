use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route("/signin", post(service::user::login))
        .route("/signout", get(service::user::logout))
        .route("/signup", post(service::user::create))
        .route("/{user_id}", delete(service::user::delete))
}
