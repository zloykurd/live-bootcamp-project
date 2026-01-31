use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn varify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}