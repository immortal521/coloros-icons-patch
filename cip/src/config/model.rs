use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 主配置结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default: DefaultConfig,
    pub sources: HashMap<String, Source>,
}

/// 默认配置部分
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultConfig {
    /// 本地图标版本
    pub icons_version: String,
    /// 更新通道，可选 stable / beta
    pub channel: String,
    /// 应用运行时目录
    pub runtime_dir: String,
    /// 本地临时下载目录
    pub temp_dir: String,
    /// 更新目标路径（最终安装目录）
    pub target_dir: String,
}

/// 每个更新通道的 index.json 信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    /// index.json 地址
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Index {
    pub channel: String,
    pub icons_version: String,
    pub revision: u32,
    pub zip_name: String,
    pub zip_sha256: String,
    pub zip_size: u64,
    pub download_url: String,
    pub published_at: String,
    pub notes: String,
}
