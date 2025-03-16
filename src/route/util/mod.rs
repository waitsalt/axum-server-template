mod captcha_image;

use axum::{Router, routing::get};

pub fn init() -> Router {
    Router::new().route(
        "/captcha_image/:captcha_image_key",
        get(captcha_image::captcha_image),
    )
}
