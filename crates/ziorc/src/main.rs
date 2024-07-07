use clap::Parser;
use config::AppStateConfig;
use state::AppState;

mod plugin_runner;
mod state;
mod web;
mod discovery;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = AppStateConfig::parse();
    let app_state = AppState::from_config(args)?;
    web::launch_axum(app_state).await?;
    Ok(())
}
