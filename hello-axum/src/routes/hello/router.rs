use axum::Json;
use axum::extract::Path;
use axum::{
    response::IntoResponse,
    routing::{get, post, put, delete},
    Router,
};

use super::service::HelloService;

use super::dtos::create_hello_request::CreateHelloRequest;
use super::dtos::update_hello_request::UpdateHelloRequest;

pub(crate) async fn get_router() -> Router {
    let router = Router::new()
        .route("/", get(get_list))
        .route("/:id", get(get_one))
        .route("/:id", post(post_one))
        .route("/:id", put(put_one))
        .route("/:id", delete(delete_one));

    router
}

async fn get_list() -> impl IntoResponse {
    let service = HelloService::new();

    let response = service.find_all();

    Json(response).into_response()
}


async fn get_one(Path(id): Path<i32>) -> impl IntoResponse {
    let service = HelloService::new();

    let response = service.find_one(id);

    Json(response).into_response()
}

async fn post_one(Json(body): Json<CreateHelloRequest>) -> impl IntoResponse {
    let service = HelloService::new();

    let response = service.create_one(body);

    Json(response).into_response()
}

async fn put_one(Path(id): Path<i32>, Json(body): Json<UpdateHelloRequest>) -> impl IntoResponse {
    let service = HelloService::new();

    let response = service.update_one(id, body);

    Json(response).into_response()
}

async fn delete_one(Path(id): Path<i32>) -> impl IntoResponse {
    let service = HelloService::new();

    let response = service.delete_one(id);

    Json(response).into_response()
}
