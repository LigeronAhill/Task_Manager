use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use log;
use log::info;
use surrealdb::engine::local::Mem;
use surrealdb::{Result, Surreal};
use tower_http::cors::CorsLayer;

use crate::routes::todos::create_router;

mod errors;
pub mod handlers;
pub mod model;
pub mod response;
pub mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("todo").use_db("todo").await?;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().with_state(db).layer(cors);

    info!("🚀 Server at http://localhost:8080 started successfully");

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
