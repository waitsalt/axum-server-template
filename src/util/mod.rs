pub mod auth;
pub mod config;
pub mod error;
pub mod logger;
pub mod response;

pub async fn init() {
    logger::init().await;
}
