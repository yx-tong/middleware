use crate::{ApiError, PoemResult};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

impl<T: Serialize, E: ApiError> IntoResponse for PoemResult<T, E> {
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
