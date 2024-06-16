// axum example
use oasgen::{OaSchema, Server, oasgen};
use axum::{Json, routing};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

/// This is the request schema for the `send-code` endpoint.
#[derive(OaSchema, Deserialize)]
pub struct SendCode {
    pub mobile: String,
}

/// This is the response schema for the `send-code` endpoint.
#[derive(Serialize, OaSchema, Debug)]
pub struct SendCodeResponse {
    pub found_account: bool,
}

/// This is the handler function for the `send-code` endpoint.
#[oasgen]
async fn send_code(_body: Json<SendCode>) -> Json<SendCodeResponse> {
    Json(SendCodeResponse { found_account: false })
}

#[tokio::main]
async fn main() {
    let server = Server::axum()
        .post("/send-code", send_code)
        .route_yaml_spec("/openapi.yaml")
        // .route_json_spec("/openapi.json")
        .swagger_ui("/openapi/")
        .freeze();

    let router = axum::Router::new()
        .route("/hello", routing::get(|| async { "OK" }))
        .merge(server.into_router());

    let server = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(server, router).await.unwrap();
    

}