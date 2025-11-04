use crate::handlers::{todos_create, todos_delete, todos_index, todos_update};
use crate::models::Db;
use axum::routing::{get, patch};
use axum::Router;

pub fn todos_router(db: Db) -> Router {
    Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/{id}", patch(todos_update).delete(todos_delete))
        .with_state(db)
}
