mod env;
mod handlers;
mod init;
mod routes;
mod shared;

#[tokio::main]
async fn main() {
    let app = init::init_app();
    lambda_http::run(app).await.unwrap();
}
