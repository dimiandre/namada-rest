use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use namada_sdk::{
    error::{self, PinnedBalanceError},
    rpc,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tendermint_rpc::{self, HttpClient};

#[tokio::main]
async fn main() {
    // Connect to RPC
    let client = Arc::new(HttpClient::new("http://127.0.0.1:26657").unwrap());

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Namada REST API Running" }))
        .route("/proposals", get(|| async { "Culo" }))
        .route("/proposals/:id", get(|| async { "Culo" }))
        .route("/epoch", get(move || get_epoch(client.clone())));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

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

pub async fn get_epoch(client: Arc<HttpClient>) -> Result<Json<Value>, MyErrorWrapper> {
    let result = tokio::task::spawn_blocking(move || {
        // Execute the blocking operation
        tokio::runtime::Handle::current().block_on(async { rpc::query_epoch(&*client).await })
    })
    .await
    .map_err(|e| {
        // Directly handle the conversion from JoinError to MyErrorWrapper
        if e.is_cancelled() {
            MyErrorWrapper(error::Error::Pinned(
                PinnedBalanceError::NoTransactionPinned,
            ))
        } else {
            // You can adjust this part to better fit your error model
            MyErrorWrapper(error::Error::Pinned(
                PinnedBalanceError::NoTransactionPinned,
            ))
        }
    })?;

    result
        .map(|epoch_data| Json(json!({ "epoch": epoch_data })))
        .map_err(MyErrorWrapper)
}
