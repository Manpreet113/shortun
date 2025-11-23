mod storage;
mod base62;
use storage::{Storage};
use axum::{
    extract::{State, Path},
    routing::{get, post},
    Json, Router, response::{IntoResponse, Redirect},
    http::StatusCode,
    http::Method,
};
use serde::{Deserialize, Serialize};
mod db;
use db::PostgresStorage;
use url::Url;
mod error;
use error::AppError;
use dotenvy::dotenv;
use std::env;
use tower_http::cors::{CorsLayer, Any};

#[derive(Deserialize)]
struct CreateRequest{
    url: String,
}

#[derive(Serialize)]
struct CreateResponse{
    slug: String,
}

async fn create_slug(State(storage): State<PostgresStorage>, Json(payload): Json<CreateRequest>) -> Result<impl IntoResponse, AppError>{

    let valid_url = validate_and_sanitize_url(&payload.url)
        .map_err(AppError::BadRequest)?;

    let id = storage.shorten(&valid_url).await?;

    let slug = format!("http://localhost:3000/{}", id);

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



#[tokio::main]
async fn main(){

    dotenv().ok();

    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST])
    .allow_headers(Any);

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());

    let storage = PostgresStorage::new(&db_url).await;

    let app = Router::new().route("/", post(create_slug)).route("/:id", get(redirect)).route("/:id/stats", get(get_url_stats)).with_state(storage).layer(cors);

    let listner = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    print!("Server chalu at http://0.0.0.0:{}", port);
    axum::serve(listner, app).await.unwrap();
}