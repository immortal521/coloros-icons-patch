mod app;
mod cli;
mod config;
mod core;

use clap::Parser;
use cli::root::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(cmd) => app::init::run(cmd)?,
        Commands::Update(cmd) => app::update::run(cmd).await?,
        Commands::Check(cmd) => app::check::run(cmd).await?,
        Commands::Scan(cmd) => app::scan::run(cmd)?,
    }

    Ok(())
}
