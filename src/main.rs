mod cli;
mod http;
mod instructions;
mod vm;

use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
    if let Args::Server = args {
        http::main().await;
    } else {
    }
    Ok(())
}
