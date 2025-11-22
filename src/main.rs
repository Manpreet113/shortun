mod storage;
mod base62;
use storage::{Storage};
use axum::{
    extract::{State, Path},
    routing::{get, post},
    Json, Router, response::{IntoResponse, Redirect},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
mod db;
use db::PostgresStorage;
use url::Url;

#[derive(Deserialize)]
struct CreateRequest{
    url: String,
}

#[derive(Serialize)]
struct CreateResponse{
    slug: String,
}

async fn create_slug(State(storage): State<PostgresStorage>, Json(payload): Json<CreateRequest>) -> impl IntoResponse{

    let valid_url = match validate_and_sanitize_url(&payload.url) {
        Ok(url) => url,
        Err(e) => return (StatusCode::BAD_REQUEST, e).into_response(), 
    };

    let id = storage.shorten(&valid_url).await;

    let slug = format!("http://localhost:3000/{}", id);

    let response = CreateResponse {slug};
    (StatusCode::CREATED, Json(response)).into_response()
}

async fn redirect(State(storage): State<PostgresStorage>, Path(id): Path<String>) -> impl IntoResponse{
    match storage.get_url(&id).await {
        Some(url) => Redirect::permanent(&url).into_response(),
        None => (StatusCode::NOT_FOUND, "URL Not Found!" ).into_response(), 
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

#[tokio::main]
async fn main(){
    let db_url = "postgres://postgres:mysecretpassword@localhost:5432/postgres";
    let storage = PostgresStorage::new(db_url).await;

    let app = Router::new().route("/", post(create_slug)).route("/:id", get(redirect)).with_state(storage);

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    print!("Server chalu at http://0.0.0.0:3000");
    axum::serve(listner, app).await.unwrap();
}