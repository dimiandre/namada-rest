use axum::{
    response::{IntoResponse, Response},
    Json,
};
use namada_sdk::error::{self, Error, PinnedBalanceError};
use serde_json::{json, Value};
use std::future::Future;

use tokio::task::JoinError;

pub struct MyErrorWrapper(error::Error);

// Implement `IntoResponse` for your new type
impl IntoResponse for MyErrorWrapper {
    fn into_response(self) -> Response {
        // Here you convert your error into an axum response.
        // You can customize this to return a JSON error message, set the status code, etc.
        let error_message = format!("{}", self.0); // Assuming error::Error implements Display
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(json!({ "error": error_message })),
        )
            .into_response()
    }
}

// Adjusted to execute a synchronous operation
pub async fn spawn_blocking_rpc<F, R>(f: F) -> Result<Json<Value>, MyErrorWrapper>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static + serde::Serialize,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|e| handle_join_error(e))
        .map(|data| Json(json!({ "data": data })))
}

// Handles conversion from JoinError to your error type
fn handle_join_error(e: JoinError) -> MyErrorWrapper {
    if e.is_cancelled() {
        MyErrorWrapper(error::Error::Pinned(
            PinnedBalanceError::NoTransactionPinned,
        ))
    } else {
        MyErrorWrapper(error::Error::Pinned(
            PinnedBalanceError::NoTransactionPinned,
        ))
    }
}
