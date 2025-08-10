mod cli;
mod client;
mod http;
mod instructions;
mod vm;

use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
    match args {
        Args::Server => {
            http::main().await;
        }
        Args::Client { ip } => {
            client::run_client(ip).await?;
        }
    }
    Ok(())
}
