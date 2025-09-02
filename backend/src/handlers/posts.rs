use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewPost {
    title: String,
    content: String,
}

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn get_posts() -> Json<Response> {
    Json(Response {
        message: "Posts endpoint".into(),
    })
}

pub async fn create_post(Json(payload): Json<NewPost>) -> Json<Response> {
    Json(Response {
        message: format!(
            "Created post: \n\t{}\n\t{}\n",
            payload.title, payload.content
        ),
    })
}
