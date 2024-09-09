use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::{ApiError, ApiResult};

impl<T: Serialize, E: ApiError> IntoResponse for ApiResult<T, E> {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap();
        Response::builder()
            // Service network always ok
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
    }
}