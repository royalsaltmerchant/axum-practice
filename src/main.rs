#![allow(unused)]

use std::net::SocketAddr;
use std::path::PathBuf;

use tower_http::services::{ServeDir, ServeFile};

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    // main routing
    let routes_hello = Router::new()
        .merge(routes_hello_static())
        .nest("/api", routes_hello_api())
        .fallback_service(routes_static());
    // serve
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

// syntax to create route group with merge
fn routes_hello_static() -> Router {
    Router::new()
        .route("/hello", get(handler_hello_query))
        .route("/hello/:name", get(handler_hello_path))
}

// syntax to create route group with nest
fn routes_hello_api() -> Router {
    Router::new()
        .route("/hello/json", get(handler_hello_json))
        .route("/hello/post", post(handler_hello_post))
}

// static using tower & fallback service
fn routes_static() -> Router {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");
    let fallback_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static/404.html");
    Router::new().nest_service(
        "/",
        ServeDir::new(assets_dir).fallback(ServeFile::new(fallback_file)),
    )
}

// serde
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// query params
async fn handler_hello_query(Query(params): Query<HelloParams>) -> Html<String> {
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<b>Hello {name}</b>"))
}
// path params
async fn handler_hello_path(Path(name): Path<String>) -> Html<String> {
    Html(format!("<b>Hello {name}</b>"))
}
// simple json return
async fn handler_hello_json() -> Json<Value> {
    Json(json!({
      "Message": "Hello Wyrld JSON",
      "Data": {"number": 69}
    }))
}

#[derive(Debug, Deserialize)]
struct HelloPostPayload {
    word: Option<String>,
    number: Option<i64>,
}

async fn handler_hello_post(Json(body): Json<HelloPostPayload>) -> Json<GenericResponse> {
    println!("{:#?}", body);
    let response_message = GenericResponse {
        message: String::from("Success"),
    };
    Json(response_message)
}

#[derive(Debug, Serialize)]
struct GenericResponse {
    message: String,
}
