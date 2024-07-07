use std::sync::Arc;

use axum::{extract::State, routing::{get, post}, Json, Router};

use crate::state::{AppState, Job, SharedAppState};


pub async fn launch_axum(state: SharedAppState) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/jobs", get(list_jobs))
        .route("/jobs", post(start_job))
        .with_state(state);

    let listener =  tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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