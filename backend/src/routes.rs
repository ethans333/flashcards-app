use crate::handlers::{posts::get_posts, users::get_users};
use axum::{Router, routing::get};

pub fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello from root" }))
        .route("/users", get(get_users))
        .route("/posts", get(get_posts))
}
