use std::path::PathBuf;

#[derive(clap::Args)]
pub struct InitCmd {
    /// Config path
    #[arg(short, long, default_value = "config.toml")]
    pub config: PathBuf,

    #[arg(long)]
    pub force: bool,

    #[arg(long, default_value = "stable")]
    pub channel: String,

    #[arg(long)]
    pub temp_dir: Option<PathBuf>,

    #[arg(long)]
    pub target_dir: Option<PathBuf>,
}
