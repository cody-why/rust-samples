use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};

use super::service::UserService;

pub(crate) async fn get_router() -> Router {
    Router::new()
        .route("/", get(get_user_list))
        .route("/:user_id", get(find_user_by_id))
}

async fn get_user_list() -> impl IntoResponse {
    let service = UserService::new();

    let response = service.find_user_list();

    Json(response).into_response()
}

async fn find_user_by_id(Path(user_id): Path<i32>) -> impl IntoResponse {
    let service = UserService::new();

    let response = service.find_user_by_id(user_id);

    Json(response).into_response()
}
