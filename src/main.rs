use std::path::PathBuf;

use axum::Router;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

use clap::Parser;

const DEFAULT_PORT: u16 = 9999;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Port to serve data from. default to 9999
    #[arg(short, long)]
    port: Option<u16>,
    /// Path to serve file from
    path: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let bind = format!("0.0.0.0:{}", args.port.unwrap_or(DEFAULT_PORT));

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();

    let app = Router::new().fallback_service(ServeDir::new(args.path));

    let listener = tokio::net::TcpListener::bind(bind).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
