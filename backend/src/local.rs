use crate::routes::app;

pub async fn run_local() {
    let addr: std::net::SocketAddr = "127.0.0.1:3000".parse().unwrap();
    println!("Running locally on http://{}/", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}
