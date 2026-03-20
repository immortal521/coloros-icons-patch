use super::model::Config;
use anyhow::{Context, Result};
use std::{env, fs, path::PathBuf};

/// 加载配置文件，直接传入 PathBuf
pub fn load(path: PathBuf) -> Result<Config> {
    let path = resolve(path)?;

    let content = fs::read_to_string(&path)
        .with_context(|| format!("failed to read config file {:?}", path))?;
    let config: Config = toml::from_str(&content)
        .with_context(|| format!("failed to parse TOML from {:?}", path))?;

    println!("Using config: {:?}", path);

    Ok(config)
}

/// 如果路径不存在，按顺序尝试 CLI / ENV / 本地 / 全局
fn resolve(path: PathBuf) -> Result<PathBuf> {
    // 1. CLI 或默认路径存在直接返回
    if path.exists() {
        return Ok(path);
    }

    // 2. 环境变量
    if let Ok(env_path) = env::var("CIP_CONFIG") {
        let p: PathBuf = env_path.into();
        if p.exists() {
            return Ok(p);
        }
    }

    // 3. 本地文件
    let local = PathBuf::from(".cip.toml");
    if local.exists() {
        return Ok(local);
    }

    // 4. 全局配置目录
    if let Some(mut global) = dirs::config_dir() {
        global.push("cip/config.toml");
        if global.exists() {
            return Ok(global);
        }
    }

    anyhow::bail!("no config found, run `cip init`");
}
