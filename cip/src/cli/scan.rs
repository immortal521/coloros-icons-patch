use std::path::PathBuf;

#[derive(clap::Args)]
pub struct ScanCmd {
    #[arg(short, long, default_value = ".")]
    pub target: PathBuf,
}
