use std::sync::Arc;

use axum::{extract::State, routing::{get, post}, Json, Router};
use mdns_sd::ServiceInfo;

use crate::{discovery::Peer, state::{AppState, Job, SharedAppState}};


pub async fn launch_axum(state: SharedAppState) -> anyhow::Result<()> {
    let port = state.read().await.config().port;
    let app = Router::new()
        .route("/", get(root))
        .route("/jobs", get(list_jobs))
        .route("/jobs", post(start_job))
        .route("/peers", get(list_peers))
        .with_state(state);

    let listener =  tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello world"
}

async fn list_jobs(state: State<SharedAppState>) -> Json<Vec<Job>> {
    Json(state.read().await.jobs())
}

async fn start_job(state: State<SharedAppState>) -> Json<Job> {
    Json(state.write().await.start_job())
}

async fn list_peers(state: State<SharedAppState>) -> Json<Vec<Peer>> {
    Json(state.read().await.peers().await.expect("Failed to find peers"))
}