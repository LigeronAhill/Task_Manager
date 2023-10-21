use crate::model::todo::{QueryOptions, Todo, TodoFromUser};
use crate::response::todo::{SingleTodoResponse, TodoData, TodoListResponse};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

fn helper(todo: surrealdb::Result<Option<Todo>>) -> Response {
    match todo {
        Ok(some_todo) => match some_todo {
            None => (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "fail",
                    "message": "Todo not found"
                })),
            )
                .into_response(),
            Some(todo) => {
                let json_response = SingleTodoResponse {
                    status: "success".to_string(),
                    data: TodoData { todo: todo.clone() },
                };
                (StatusCode::OK, Json(json_response)).into_response()
            }
        },
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "status": "fail",
                "message": "Todo not found"
            })),
        )
            .into_response(),
    }
}

pub async fn create_todo_handler(
    State(db): State<Surreal<Db>>,
    Json(todo): Json<TodoFromUser>,
) -> impl IntoResponse {
    let todo = todo.build().await;
    let result: surrealdb::Result<Vec<Todo>> = db.create(todo.id.tb.clone()).content(todo).await;
    match result {
        Ok(todo_slice) => {
            let json_response = SingleTodoResponse {
                status: "success".to_string(),
                data: TodoData {
                    todo: todo_slice[0].clone(),
                },
            };
            (StatusCode::CREATED, Json(json_response)).into_response()
        }
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "fail",
                    "message": "Todo not found"
                })),
            )
                .into_response()
        }
    }
}

pub async fn todos_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<Surreal<Db>>,
) -> impl IntoResponse {
    let todos: surrealdb::Result<Vec<Todo>> = db.select("todo").await;
    match todos {
        Ok(todos) => {
            let Query(opts) = opts.unwrap_or_default();
            let limit = opts.limit.unwrap_or(10);
            let offset = (opts.page.unwrap_or(1) - 1) * limit;
            let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();
            let json_response = TodoListResponse {
                status: "success".to_string(),
                results: todos.len(),
                todos,
            };

            Json(json_response).into_response()
        }
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "fail",
                    "message": "Todo not found"
                })),
            )
                .into_response()
        }
    }
}

pub async fn get_todo_handler(
    State(db): State<Surreal<Db>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let todo: surrealdb::Result<Option<Todo>> = db.select(("todo", id)).await;
    helper(todo)
}

pub async fn edit_todo_handler(
    State(db): State<Surreal<Db>>,
    Path(id): Path<String>,
    Json(body): Json<TodoFromUser>,
) -> impl IntoResponse {

    let new_todo = body.build().await;

    let updated: surrealdb::Result<Option<Todo>> = db
        .update(("todo", id.clone())).content(new_todo).await;

    helper(updated)
}

pub async fn delete_todo_handler(
    State(db): State<Surreal<Db>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let deleted: surrealdb::Result<Option<Todo>> = db.delete(("todo", id)).await;
    helper(deleted)
}
