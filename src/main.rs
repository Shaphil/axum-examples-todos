mod handlers;
mod models;

use crate::handlers::todos_update;
use crate::models::Db;
use axum::{Router, routing::get, routing::patch, routing::post};
use handlers::todos_create;
use handlers::todos_index;

#[tokio::main]
async fn main() {
    let db = Db::default();

    let app = Router::new()
        .route("/", get(todos_index))
        .route("/", post(todos_create))
        .route("/todos/{id}", patch(todos_update))
        .with_state(db);
    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 3000;
    let address = format!("{}:{}", HOST, PORT);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Server running at http://{HOST}:{PORT}");

    axum::serve(listener, app).await.unwrap();
}
