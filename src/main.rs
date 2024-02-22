use axum::{
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::process;
use std::{fs};
use std::path::Path;
use utils::{get_epoch, get_proposals};
use tendermint_rpc::{self, HttpClient};
use tower_http::cors::{CorsLayer, Any};

mod utils;


#[derive(Clone, Debug, Serialize, Deserialize)]
struct Settings {
    rpc_url: String,
    bind_ip: String,
    port: u16,
}

#[derive(Clone)]
pub struct ServerState {
    client: HttpClient,
    config: Settings
}

#[tokio::main]
async fn main() {

    // Load config
    let settings_path = "config/Settings.toml";
    let config = read_settings_from_file(settings_path).unwrap_or_else(|err| {
        eprintln!("Failed to read settings: {}", err);
        process::exit(1);
    });

    // Connect to RPC
    let client = HttpClient::new(config.rpc_url.as_str()).unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Namada REST API Running" }))
        .route("/proposal_result/:id", get(get_proposals))
        .route("/epoch", get(get_epoch))
        .with_state(ServerState { client: client, config: config.clone() })
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_credentials(false),
        );

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.bind_ip, config.port)).await.unwrap();
    println!("Server listening {}:{}", config.bind_ip, config.port);

    axum::serve(listener, app).await.unwrap();
}


fn read_settings_from_file<P: AsRef<Path>>(path: P) -> Result<Settings, Box<dyn std::error::Error>> {
    let settings_str = fs::read_to_string(path)?;
    let settings: Settings = toml::from_str(&settings_str)?;
    Ok(settings)
}