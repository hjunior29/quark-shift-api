use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber;

mod db;
mod handlers;
mod models;
mod response;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .init();

    let db = db::connect().await.expect("Failed to connect to database");

    let app = routes::create_routes(db);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
