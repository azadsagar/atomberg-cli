use clap::Parser;
use cli::args::Cli;

use crate::cli::handlers::run;

mod cli;
mod config;
mod device;
mod network;
mod protocol;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    run(cli).await?;
    Ok(())
}
