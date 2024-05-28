use std::time::Duration;

use axum::body::{Body, BoxBody};
use axum::Json;
use axum::{
    http::{Request, Response},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;
use tracing::Span;

pub(crate) async fn get_router() -> Router {
    let trace = TraceLayer::new_for_http()
        .on_request(|request: &Request<Body>, _span: &Span| {
            println!("{} {} started", request.method(), request.uri().path());
            println!("request: {request:?}",);
        })
        .on_response(
            |response: &Response<BoxBody>, latency: Duration, _span: &Span| {
                println!("response generated in {latency:?}",);
                println!("response: {response:?}",);
            },
        );

    // Root Routing
    let router = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .nest("/user", crate::routes::user::router::get_router().await)
        .nest("/hello", crate::routes::hello::router::get_router().await)
        // Append the new route here. (don't delete this comment)
        .layer(trace);

    router
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

use super::service::RootService;

async fn health() -> impl IntoResponse {
    let service = RootService::new();

    let response = service.get_health();

    Json(response).into_response()
}