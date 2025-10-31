mod handlers;
mod models;

use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::{routing::get, routing::patch, Router};
use handlers::{todos_create, todos_delete, todos_index, todos_update};
use models::Db;
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let db = Db::default();

    let app = Router::new()
        .route("/", get(todos_index).post(todos_create))
        .route("/todos/{id}", patch(todos_update).delete(todos_delete))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(db);
    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 3000;
    let address = format!("{}:{}", HOST, PORT);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Server running at http://{HOST}:{PORT}");

    axum::serve(listener, app).await.unwrap();
}
