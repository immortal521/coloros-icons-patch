use std::path::PathBuf;

#[derive(clap::Args)]
pub struct CheckCmd {
    /// Config path
    #[arg(short, long, default_value = "config.toml")]
    pub config: PathBuf,

    #[arg(long)]
    pub json: bool,
}
