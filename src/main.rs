use axum::{
    routing::get,
    Router,
};

use utils::{get_epoch, get_proposals};
use tendermint_rpc::{self, HttpClient};

mod utils;

#[derive(Clone)]
pub struct ServerState {
    client: HttpClient,
}

#[tokio::main]
async fn main() {
    // Connect to RPC
    let client = HttpClient::new("https://rpc-namada.kintsugi-nodes.com").unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Namada REST API Running" }))
        .route("/proposal_result/:id", get(get_proposals))
        .route("/epoch", get(get_epoch))
        .with_state(ServerState { client: client });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

