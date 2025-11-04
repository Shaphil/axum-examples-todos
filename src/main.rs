mod handlers;
mod logging;
mod models;
mod routes;

use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use logging::init_log;
use models::Db;
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Initialize logging infrastructure
    init_log();

    // Log the application start (courtesy of our tracing setup)
    tracing::info!("Initializing Todo API...");

    let db = Db::default();
    let app_routes = routes::todos_router(db);
    let app = app_routes.layer(
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
            // tracing and logging for http requests
            .layer(TraceLayer::new_for_http())
            .into_inner(),
    );

    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 3000;
    let address = format!("{}:{}", HOST, PORT);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Server running at http://{HOST}:{PORT}");

    axum::serve(listener, app).await.unwrap();
}
