use axum::{response::Html, routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

//  $env:RUST_LOG="debug"; cargo run --bin web    启动命令
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // let app = Router::new().route("/", get(handler));
    // axum::serve(listener, app).await.unwrap();

    let serve_dir = ServeDir::new("assets")
        .not_found_service(ServeFile::new("assets/index.html"));
    let static_app = Router::new()
        .route("/foo", get(handler))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/assets2", serve_dir.clone())
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9999")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, static_app).await.unwrap();

}

async fn handler() -> Html<&'static str> {
    Html("hello rust web")
}