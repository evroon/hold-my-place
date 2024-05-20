use std::fs::read_to_string;

use axum::{
    extract::{Path, Query},
    http::header,
    response::IntoResponse,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
mod models;
mod render;
mod svg;
mod text;
mod text_builder;
use models::Font;
use serde::Deserialize;
use tokio::task;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/:width/:height", get(image_handler))
        .nest_service("/assets", ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3300")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct ImageQueryParams {
    text: Option<String>,
    #[serde(default = "default_color")]
    color: String,
    #[serde(default = "default_background_color")]
    background: String,
    #[serde(default = "default_font")]
    font: Font,
}

fn default_color() -> String {
    String::from("#999999")
}

fn default_background_color() -> String {
    String::from("#dddddd")
}

fn default_font() -> Font {
    Font::Lato
}

async fn index_handler() -> impl IntoResponse {
    if let Ok(source) = read_to_string("assets/index.html") {
        let source_substituted = source.replace("${address}", "http://localhost:3300");
        ([(header::CONTENT_TYPE, "text/html")], source_substituted)
    } else {
        (
            [(header::CONTENT_TYPE, "text/plain")],
            String::from("Could not read index.html"),
        )
    }
}

async fn image_handler(
    Path((width, height)): Path<(u32, u32)>,
    query_params: Query<ImageQueryParams>,
) -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/png")],
        task::spawn_blocking(move || {
            render::render(
                width,
                height,
                &match &query_params.text {
                    Some(x) => x.to_string(),
                    None => format!("{} x {}", width, height),
                },
                &query_params.0.color,
                &query_params.0.background,
                query_params.font,
            )
        })
        .await
        .unwrap(),
    )
}
