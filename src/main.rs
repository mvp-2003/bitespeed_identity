#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use sqlx::PgPool;

mod models;
mod repo;
mod service;
mod error;

use models::{IdentifyRequest, IdentifyResponse};
use error::ApiResult;

#[post("/identify", format = "json", data = "<payload>")]
async fn identify_route(payload: Json<IdentifyRequest>, pool: &rocket::State<PgPool>) -> ApiResult<Json<IdentifyResponse>> {
    let body = payload.into_inner();
    let summary = service::identify(pool, body).await?;
    Ok(Json(IdentifyResponse { contact: summary }))
}

#[get("/health")]
async fn health() -> &'static str {
    "ok"
}

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_shared_db::Postgres] database_url: String
) -> shuttle_rocket::ShuttleRocket {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let rocket = rocket::build()
        .manage(pool)
        .mount("/", routes![health, identify_route]);
    Ok(rocket.into())
}