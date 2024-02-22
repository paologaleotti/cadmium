use tracing::info;

mod handlers;
mod init;
mod routes;
mod shared;

#[tokio::main]
async fn main() {
    let port = "3000";
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let app = init::init_app();

    info!("server started at http://localhost:{}", port);
    axum::serve(listener, app).await.unwrap();
}
