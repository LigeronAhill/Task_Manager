use axum::response::IntoResponse;
use axum::Json;

pub mod todos;

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Health Ok!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });
    Json(json_response)
}
