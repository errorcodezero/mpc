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
        Args::Server { ip, port }  => {
            http::main(ip, port).await;
        }
        Args::Client { ip, port } => {
            client::run_client(ip, port).await?;
        }
    }
    Ok(())
}
