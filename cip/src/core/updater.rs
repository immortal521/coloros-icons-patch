use crate::config::model::Config;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::io::Cursor;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{fs, path::PathBuf};
use zip::ZipArchive;

/// 对应 index.json
#[derive(Deserialize, Debug)]
struct Index {
    icons_version: String,
    zip_name: String,
    zip_sha256: String,
    // zip_size: u64,
    download_url: String,
}

/// 下载文件到指定路径
async fn download_file(url: &str, dest: &PathBuf) -> Result<()> {
    println!("Downloading {} -> {:?}", url, dest);
    let client = Client::new();
    let resp = client.get(url).send().await?.bytes().await?;
    fs::write(dest, &resp)?;
    Ok(())
}

/// 校验文件 SHA256
fn verify_sha256(path: &PathBuf, expected: &str) -> Result<()> {
    let data = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let hash = format!("{:x}", hasher.finalize());
    if hash != expected {
        return Err(anyhow!("SHA256 mismatch: {} != {}", hash, expected));
    }
    Ok(())
}

/// 增量解压 ZIP 并设置权限
fn unzip_file_incremental(zip_path: &Path, target_dir: &Path) -> Result<()> {
    let data = fs::read(zip_path)?;
    let mut archive = ZipArchive::new(Cursor::new(data))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = target_dir.join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
            let mut perms = fs::metadata(&outpath)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&outpath, perms)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
                let mut perms = fs::metadata(parent)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(parent, perms)?;
            }

            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;

            let mut perms = outfile.metadata()?.permissions();
            perms.set_mode(0o644);
            fs::set_permissions(&outpath, perms)?;
        }
    }

    Ok(())
}

/// 核心 updater
pub async fn run(config: Config, download_base: bool) -> Result<()> {
    if download_base {
        println!("download_base")
    }
    // 下载 base.zip（如果需要）
    // if download_base {
    //     if let Some(base) = &config.base {
    //         let base_zip_path = PathBuf::from(&config.default.temp_dir).join(&base.zip_name);
    //         download_file(&base.download_url, &base_zip_path).await?;
    //         verify_sha256(&base_zip_path, &base.zip_sha256)?;
    //         unzip_file_incremental(&base_zip_path, &PathBuf::from(&config.default.target_dir))?;
    //         println!("Base.zip applied");
    //     }
    // }

    // 更新 index.zip
    let channel = &config.default.channel;
    let source = config
        .sources
        .get(channel)
        .ok_or_else(|| anyhow!("source '{}' not found", channel))?;

    println!("Fetching index: {}", source.url);

    let index: Index = reqwest::get(&source.url).await?.json().await?;
    println!("Latest version: {}", index.icons_version);

    let zip_path = PathBuf::from(&config.default.temp_dir).join(&index.zip_name);

    download_file(&index.download_url, &zip_path).await?;
    verify_sha256(&zip_path, &index.zip_sha256)?;
    unzip_file_incremental(&zip_path, &PathBuf::from(&config.default.target_dir))?;

    println!("Update applied to {:?}", config.default.target_dir);
    Ok(())
}
