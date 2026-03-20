use clap::{Parser, Subcommand};

use crate::cli::{check::CheckCmd, config::ConfigCmd, scan::ScanCmd};

use super::{init::InitCmd, update::UpdateCmd};

#[derive(Parser)]
#[command(name = "cip")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a config
    Init(InitCmd),

    /// Check for updates
    Check(CheckCmd),

    /// Update icons
    Update(UpdateCmd),

    Scan(ScanCmd),

    Config {
        #[command(subcommand)]
        cmd: ConfigCmd,
    },
}
