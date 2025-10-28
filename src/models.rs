use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct Todo {
    id: Uuid,
    text: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    text: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}
