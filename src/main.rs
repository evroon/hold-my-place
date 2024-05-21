use std::{env, fs::read_to_string};

use axum::{
    extract::{Path, Query},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use params::ImageQueryParams;
use tower_http::services::{ServeDir, ServeFile};
mod models;
mod params;
mod rendering;
use tokio::task;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let address = match env::var_os("HOLD_MY_PLACE_PORT") {
        Some(v) => v.into_string().unwrap(),
        None => String::from("0.0.0.0:3300"),
    };

    let assets_service = ServeDir::new("assets").fallback(ServeFile::new("assets/404.html"));

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/:width", get(image_handler_squared))
        .route("/:width/:height", get(image_handler_complete))
        .nest_service("/assets", assets_service)
        .fallback(handler_404);

    let listener = tokio::net::TcpListener::bind(address.clone())
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
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

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "text/html")],
        include_str!("../assets/404.html"),
    )
}

async fn image_handler_squared(
    Path(width): Path<u32>,
    query_params: Query<ImageQueryParams>,
) -> impl IntoResponse {
    image_handler((width, width), query_params).await
}

async fn image_handler_complete(
    Path((width, height)): Path<(u32, u32)>,
    query_params: Query<ImageQueryParams>,
) -> impl IntoResponse {
    image_handler((width, height), query_params).await
}

async fn image_handler(
    (width, height): (u32, u32),
    query_params: Query<ImageQueryParams>,
) -> impl IntoResponse {
    let content_type = query_params.filetype.get_content_type();
    let (width, height) = (width.max(10).min(3200), height.max(10).min(3200));

    let result = task::spawn_blocking(move || {
        query_params.filetype.get_render_func()(
            width,
            height,
            &match &query_params.text {
                Some(x) => x.to_string(),
                None => format!("{} x {}", width, height),
            },
            &query_params.color,
            &query_params.background,
            query_params.font,
        )
    })
    .await;

    if let Ok(image) = result {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, content_type)],
            image.unwrap(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, String::from("text/plain"))],
            "Unexpected error".as_bytes().into(),
        )
    }
}
