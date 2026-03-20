use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct UpdateCmd {
    /// 配置文件路径，默认 config.toml
    #[arg(long, default_value = "config.toml")]
    pub config: PathBuf,

    /// 是否下载 base.zip（如果配置了 base）
    #[arg(long)]
    pub download_base: bool,
}
