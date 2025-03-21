mod captcha_email;
mod captcha_image;
mod captcha_phone;

use axum::{Router, routing::get};

pub fn init() -> Router {
    Router::new()
        .route("/", get(root))
        .route(
            "/captcha_email/{user_email}",
            get(captcha_email::captcha_email),
        )
        .route(
            "/captcha_image/{captcha_image_key}",
            get(captcha_image::captcha_image),
        )
}

async fn root() -> &'static str {
    return "This is util router.";
}
