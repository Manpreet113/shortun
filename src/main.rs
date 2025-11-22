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

#[derive(Deserialize)]
struct CreateRequest{
    url: String,
}

#[derive(Serialize)]
struct CreateResponse{
    slug: String,
}

async fn create_slug(State(storage): State<PostgresStorage>, Json(payload): Json<CreateRequest>) -> impl IntoResponse{
    let id = storage.shorten(&payload.url).await;

    let slug = format!("http://localhost:3000/{}", id);

    let response = CreateResponse {slug};
    (StatusCode::CREATED, Json(response))
}

async fn redirect(State(storage): State<PostgresStorage>, Path(id): Path<String>) -> impl IntoResponse{
    match storage.get_url(&id).await {
        Some(url) => Redirect::permanent(&url).into_response(),
        None => (StatusCode::NOT_FOUND, "URL Not Found!" ).into_response(), 
    }
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