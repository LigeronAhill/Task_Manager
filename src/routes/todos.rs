use axum::routing::{get, post};
use axum::Router;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

use crate::handlers::todo::{
    create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler,
    todos_list_handler,
};
use crate::routes::health_checker_handler;

pub fn create_router() -> Router<Surreal<Db>> {
    let todo_routes = Router::new()
        .route("/", post(create_todo_handler).get(todos_list_handler))
        .route(
            "/:id",
            get(get_todo_handler)
                .patch(edit_todo_handler)
                .delete(delete_todo_handler),
        );

    Router::new()
        .nest("/api/v1/todos", todo_routes)
        .route("/api/v1/health", get(health_checker_handler))
}
