use clap::Parser;
use cli::args::Cli;

use crate::cli::handlers::run;

mod cli;
mod config;
mod network;
mod protocol;
mod utils;
mod device;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let _ = run(cli).await;
}
