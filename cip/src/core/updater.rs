use crate::config::model::Config;
use anyhow::{anyhow, Result};
use futures_util::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::io::{self, Write};
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
    download_url: String,
}

/// 统一输出
fn emit(json_mode: bool, value: serde_json::Value) {
    if json_mode {
        println!("{}", value);
    } else if let Some(msg) = value.get("message").and_then(|v| v.as_str()) {
        println!("{}", msg);
    }
    io::stdout().flush().ok();
}

/// 下载文件
async fn download_file(url: &str, dest: &PathBuf, json_mode: bool) -> Result<()> {
    emit(json_mode, json!({ "type": "stage", "value": "download" }));

    let resp = Client::new().get(url).send().await?;

    if !resp.status().is_success() {
        return Err(anyhow!("HTTP error: {}", resp.status()));
    }

    let total = resp.content_length().unwrap_or(0);
    let mut downloaded = 0u64;
    let mut last = 0;

    let mut file = fs::File::create(dest)?;
    let mut stream = resp.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;

        if total == 0 {
            continue;
        }

        let percent = downloaded * 100 / total;

        if percent < last + 2 {
            continue;
        }

        last = percent;

        emit(
            json_mode,
            json!({
                "type": "progress",
                "stage": "download",
                "value": percent
            }),
        );
    }

    Ok(())
}

/// 校验 SHA256
fn verify_sha256(path: &PathBuf, expected: &str, json_mode: bool) -> Result<()> {
    emit(
        json_mode,
        json!({
            "type": "stage",
            "value": "verify",
            "message": "Verifying SHA256"
        }),
    );

    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    let hash = format!("{:x}", hasher.finalize());

    if hash != expected {
        return Err(anyhow!("SHA256 mismatch: {} != {}", hash, expected));
    }

    emit(
        json_mode,
        json!({
            "type": "info",
            "value": "verify_ok",
            "message": "SHA256 OK"
        }),
    );

    Ok(())
}

/// 设置权限工具
fn set_perm(path: &Path, mode: u32) -> Result<()> {
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(mode);
    fs::set_permissions(path, perms)?;
    Ok(())
}

/// 解压
fn unzip_file_incremental(zip_path: &Path, target_dir: &Path, json_mode: bool) -> Result<()> {
    emit(json_mode, json!({ "type": "stage", "value": "extract" }));

    let file = fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    let total = archive.len();

    for i in 0..total {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();
        let outpath = target_dir.join(&name);

        let percent = (i + 1) * 100 / total;

        emit(
            json_mode,
            json!({
                "type": "progress",
                "stage": "extract",
                "value": percent,
                "file": name
            }),
        );

        if entry.is_dir() {
            fs::create_dir_all(&outpath)?;
            set_perm(&outpath, 0o755)?;
            continue;
        }

        if let Some(parent) = outpath.parent() {
            fs::create_dir_all(parent)?;
            set_perm(parent, 0o755)?;
        }

        let mut outfile = fs::File::create(&outpath)?;
        std::io::copy(&mut entry, &mut outfile)?;

        set_perm(&outpath, 0o644)?;
    }

    Ok(())
}

/// 核心 updater
pub async fn run(config: Config, download_base: bool, json_mode: bool) -> Result<()> {
    if download_base {
        emit(
            json_mode,
            json!({
                "type": "info",
                "message": "download_base enabled"
            }),
        );
    }

    let channel = &config.default.channel;
    let source = config
        .sources
        .get(channel)
        .ok_or_else(|| anyhow!("source '{}' not found", channel))?;

    emit(
        json_mode,
        json!({
            "type": "stage",
            "value": "fetch",
            "message": format!("Fetching index: {}", source.url)
        }),
    );

    let index: Index = reqwest::get(&source.url).await?.json().await?;

    emit(
        json_mode,
        json!({
            "type": "info",
            "value": "version",
            "version": index.icons_version,
            "message": format!("Latest version: {}", index.icons_version)
        }),
    );

    let zip_path = PathBuf::from(&config.default.temp_dir).join(&index.zip_name);

    download_file(&index.download_url, &zip_path, json_mode).await?;
    verify_sha256(&zip_path, &index.zip_sha256, json_mode)?;
    unzip_file_incremental(
        &zip_path,
        &PathBuf::from(&config.default.target_dir),
        json_mode,
    )?;

    emit(
        json_mode,
        json!({
            "type": "done",
            "target": config.default.target_dir
        }),
    );

    Ok(())
}
