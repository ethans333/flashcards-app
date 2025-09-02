use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn get_users() -> Json<Response> {
    Json(Response {
        message: "Users endpoint".into(),
    })
}
