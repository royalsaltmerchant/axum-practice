#![allow(unused)]

use axum::extract::{Path, Query};
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::{ServeDir, ServeFile};

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    // main routing
    let routes_hello = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());
    // serve
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

// syntax to create route group with merge
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello_query))
        .route("/hello/:name", get(handler_hello_path))
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
