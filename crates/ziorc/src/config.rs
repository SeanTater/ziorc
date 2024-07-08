use clap::Parser;

/// Zero Impedence Orchestrator
#[derive(Parser)]
pub struct AppStateConfig {
    /// Name of the application, to separate multiple apps on the same network
    /// during application discovery. Should have under about 32 characters and be alphanumeric
    pub app_name: String,

    /// Port to run the API server on
    #[arg(long)]
    pub port: Option<u16>,
}
