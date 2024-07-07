use state::AppState;

mod plugin_runner;
mod state;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let app_state = AppState::from_name("example.com");
    web::launch_axum(app_state).await?;
    Ok(())
}
