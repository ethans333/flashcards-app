mod handlers;
mod lambda;
mod local;
mod routes;

#[tokio::main]
async fn main() {
    if std::env::var("LAMBDA_TASK_ROOT").is_ok() {
        lambda::run_lambda().await;
    } else {
        local::run_local().await;
    }
}
