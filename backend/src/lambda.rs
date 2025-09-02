use crate::routes::app;
use axum::body::to_bytes;
use lambda_http::{Body as LambdaBody, Error, Request, Response, service_fn};
use tower::ServiceExt;

pub async fn run_lambda() {
    let app = app();

    lambda_http::run(service_fn(|request: Request| {
        let app = app.clone();
        async move {
            let response = app.oneshot(request).await?;
            let (parts, body) = response.into_parts();

            // Convert Axum body into Lambda body
            let bytes = to_bytes(body, 2 * 1024 * 1024).await?;
            let lambda_body = LambdaBody::from(bytes.to_vec());

            Ok::<Response<LambdaBody>, Error>(Response::from_parts(parts, lambda_body))
        }
    }))
    .await
    .unwrap();
}
