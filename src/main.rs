mod storage;
mod base62;
use storage::{Storage};
use shuttle_axum::axum::{
    extract::{State, Path},
    routing::{get, post},
    Json, Router, response::{IntoResponse, Redirect},
    http::StatusCode,
    http::HeaderMap,
};
use serde::{Deserialize, Serialize};
mod db;
use db::PostgresStorage;
use url::Url;
mod error;
use error::AppError;
use std::env;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct CreateRequest{
    url: String,
}

#[derive(Serialize)]
struct CreateResponse{
    slug: String,
}

async fn create_slug(State(storage): State<PostgresStorage>, headers: HeaderMap, Json(payload): Json<CreateRequest>) -> Result<impl IntoResponse, AppError>{

    let valid_url = validate_and_sanitize_url(&payload.url)
        .map_err(AppError::BadRequest)?;

    let id = storage.shorten(&valid_url).await?;

    let host = headers
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost:8000");

    let slug = format!("http://{}/{}", host, id);

    let response = CreateResponse {slug};
    Ok((StatusCode::CREATED, Json(response)).into_response())
}

async fn redirect(State(storage): State<PostgresStorage>, Path(id): Path<String>) -> Result<impl IntoResponse, AppError>{

    let url_option = storage.get_url(&id).await?;

    match url_option {
        Some(url) => Ok(Redirect::permanent(&url).into_response()),
        None => Err(AppError::NotFound),
    }
}

fn validate_and_sanitize_url(raw_url: &str) -> Result<String, String> {
    let url_to_parse = if raw_url.starts_with("http://") || raw_url.starts_with("https://") {
        raw_url.to_string()
} else {
    format!("https://{}", raw_url)
    };

    let parsed = Url::parse(&url_to_parse).map_err(|_| "Invalid URL format".to_string())?;

    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err("Only HTTP and HTTPS schemes are allowed".to_string());
    }

    Ok(parsed.to_string())
}

#[derive(Serialize)]
struct StatsResponse {
    clicks: i64,
}

async fn get_url_stats(
    State(storage): State<PostgresStorage>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    match storage.get_stats(&id).await? {
        Some(clicks) => Ok(Json(StatsResponse { clicks })),
        None => Err(AppError::NotFound),
    }
}



#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    let storage = PostgresStorage { pool }; 

    let app = Router::new()
        .route("/api/shorten", post(create_slug))
        .route("/{id}", get(redirect))
        .route("/{id}/stats", get(get_url_stats))
        .with_state(storage)
        .layer(CorsLayer::permissive()); // Permissive cause it ain't no prod. i just want it up, sowwy

    Ok(app.into())
}