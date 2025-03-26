use axum::{Router, routing::get};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route(
            "/captcha_email/{user_email}",
            get(service::util::captcha::email),
        )
        .route(
            "/captcha_image/{captcha_image_key}",
            get(service::util::captcha::image),
        )
}
