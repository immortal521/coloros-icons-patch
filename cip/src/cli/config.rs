use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum ConfigCmd {
    Get(GetCmd),
    Set(SetCmd),
}

#[derive(Args, Debug)]
pub struct GetCmd {
    #[arg(long, default_value = "config.toml")]
    pub config: PathBuf,

    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct SetCmd {
    #[arg(long, default_value = "config.toml")]
    pub config: PathBuf,

    #[arg(long)]
    pub channel: Option<String>,

    #[arg(long)]
    pub target_dir: Option<PathBuf>,

    #[arg(long)]
    pub temp_dir: Option<PathBuf>,

    #[arg(long)]
    pub icons_version: Option<String>,
}
