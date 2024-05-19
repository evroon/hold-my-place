use axum::{
    extract::{Path, Query},
    http::header,
    response::IntoResponse,
    routing::get,
    Router,
};
mod models;
mod render;
mod svg;
mod text;
mod text_builder;
use models::Font;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/:width/:height", get(image_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3300")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Hello, World 2!"
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

async fn image_handler(
    Path((width, height)): Path<(u32, u32)>,
    query_params: Query<ImageQueryParams>,
) -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/png")],
        render::render(
            width,
            height,
            &match &query_params.text {
                Some(x) => x.to_string(),
                None => format!("{}x{}", width, height),
            },
            &query_params.0.color,
            &query_params.0.background,
            query_params.font,
        ),
    )
}
