use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn get_posts() -> Json<Response> {
    Json(Response {
        message: "Posts endpoint".into(),
    })
}
