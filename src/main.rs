mod handlers;
mod models;

use handlers::todos_index;

use crate::models::Db;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let db = Db::default();

    let app = Router::new().route("/", get(todos_index)).with_state(db);
    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 3000;
    let address = format!("{}:{}", HOST, PORT);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Server running at http://{HOST}:{PORT}");

    axum::serve(listener, app).await.unwrap();
}
