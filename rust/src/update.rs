use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::settings::Settings;
use crate::state::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub channel: String,
    pub icons_version: String,
    pub revision: i64,
    pub zip_name: String,
    pub zip_sha256: String,
    pub zip_size: u64,
    pub download_url: String,
    pub published_at: String,
    pub notes: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateResult {
    pub checked: bool,
    pub applied: bool,
    pub message: String,

    pub index_url: String,
    pub moddir: String,

    pub remote: Option<Index>,
    pub downloaded_bytes: Option<u64>,
    pub sha256_ok: Option<bool>,
    pub size_ok: Option<bool>,
}

fn settings_path(moddir: &str) -> String {
    format!("{}/runtime/settings.json", moddir.trim_end_matches('/'))
}
fn state_path(moddir: &str) -> String {
    format!("{}/runtime/state.json", moddir.trim_end_matches('/'))
}

fn sha256_file(p: &Path) -> Result<String> {
    let mut f = fs::File::open(p)?;
    let mut h = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(hex::encode(h.finalize()))
}

fn unzip_to(zip_path: &Path, out_dir: &Path) -> Result<()> {
    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let f = fs::File::open(zip_path)?;
    let mut z = zip::ZipArchive::new(f).context("open zip failed")?;
    for i in 0..z.len() {
        let mut file = z.by_index(i)?;
        let name = file.name().to_string();

        // 防 ZipSlip
        let rel = Path::new(&name);
        if rel
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            bail!("zip contains invalid path: {}", name);
        }

        let outpath = out_dir.join(rel);

        if name.ends_with('/') {
            fs::create_dir_all(&outpath)?;
            continue;
        }

        if let Some(p) = outpath.parent() {
            fs::create_dir_all(p)?;
        }
        let mut out = fs::File::create(&outpath)?;
        std::io::copy(&mut file, &mut out)?;
    }
    Ok(())
}

fn find_staging_hdpi(staging_root: &Path) -> Result<PathBuf> {
    // 支持两种结构：
    // A) uxicons/hdpi/...
    // B) hdpi/...
    let a = staging_root.join("uxicons").join("hdpi");
    if a.is_dir() {
        return Ok(a);
    }
    let b = staging_root.join("hdpi");
    if b.is_dir() {
        return Ok(b);
    }
    bail!(
        "zip layout invalid: no (uxicons/hdpi) or (hdpi) found under {}",
        staging_root.display()
    );
}

fn remove_path(p: &Path) -> Result<()> {
    if !p.exists() {
        return Ok(());
    }
    if p.is_dir() {
        fs::remove_dir_all(p)?;
    } else {
        fs::remove_file(p)?;
    }
    Ok(())
}

/// 清空目录内容（不删除目录本体）
/// 用于回滚：确保 bind mount 源目录 inode 不变
fn clear_dir_contents(dir: &Path) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for ent in fs::read_dir(dir).with_context(|| format!("read_dir failed: {}", dir.display()))? {
        let p = ent?.path();
        remove_path(&p)?;
    }
    Ok(())
}

/// 把 src_dir 下的一级子项移动到 dst_dir（同一文件系统内 fs::rename 很快）
/// 注意：只移动子项，不替换 src_dir / dst_dir 目录本体（避免 bind mount 失效）
fn move_children(src_dir: &Path, dst_dir: &Path) -> Result<()> {
    fs::create_dir_all(dst_dir)?;
    if !src_dir.is_dir() {
        return Ok(());
    }

    for ent in
        fs::read_dir(src_dir).with_context(|| format!("read_dir failed: {}", src_dir.display()))?
    {
        let ent = ent?;
        let src = ent.path();
        let name = ent.file_name();
        let dst = dst_dir.join(name);

        // dst 存在则先删掉，避免 rename 失败
        remove_path(&dst)
            .with_context(|| format!("remove existing dst failed: {}", dst.display()))?;

        fs::rename(&src, &dst)
            .with_context(|| format!("rename failed: {} -> {}", src.display(), dst.display()))?;
    }

    Ok(())
}

/// 将 staging 的 hdpi 内容应用到 live hdpi（保持 live hdpi inode 不变，适配 bind mount）
/// - 旧内容先移到 backup/hdpi-<ts>
/// - 新内容移入 live/uxicons/hdpi
/// - 失败则清空 live 并从 backup 恢复（backup 不混入残留新内容）
fn apply_into_bound_hdpi(moddir: &str, staging_root: &Path, backup_root: &Path) -> Result<()> {
    let live_hdpi = PathBuf::from(moddir).join("uxicons").join("hdpi");
    fs::create_dir_all(&live_hdpi)
        .with_context(|| format!("mkdir live hdpi: {}", live_hdpi.display()))?;

    // staging hdpi
    let staging_hdpi = find_staging_hdpi(staging_root)?;

    // 备份目录：runtime/backup/hdpi-<ts>
    let ts = time::OffsetDateTime::now_utc().unix_timestamp();
    let backup_hdpi = backup_root.join(format!("hdpi-{}", ts));
    fs::create_dir_all(&backup_hdpi)
        .with_context(|| format!("mkdir backup hdpi: {}", backup_hdpi.display()))?;

    // 1) 先把 live 的旧内容挪到 backup（不动 live_hdpi 目录本体）
    move_children(&live_hdpi, &backup_hdpi).context("backup live hdpi children failed")?;

    // 2) 再把 staging 的新内容挪到 live
    if let Err(e) =
        move_children(&staging_hdpi, &live_hdpi).context("apply staging hdpi into live failed")
    {
        // 3) 失败回滚：清空 live，再从 backup 恢复
        let _ = clear_dir_contents(&live_hdpi);
        let _ = move_children(&backup_hdpi, &live_hdpi);
        return Err(e);
    }

    Ok(())
}

pub fn run_update(
    index_url_cli: Option<&str>,
    moddir: &str,
    apply: bool,
    verbose: bool,
) -> Result<UpdateResult> {
    let moddir = moddir.trim_end_matches('/').to_string();

    // 读取 settings.json
    let cfg: Settings =
        Settings::load_or_default(&settings_path(&moddir)).context("load settings.json failed")?;

    let index_url = match index_url_cli {
        Some(s) if !s.trim().is_empty() => s.trim().to_string(),
        _ => cfg.index_url.clone().unwrap_or_default(),
    };

    if index_url.is_empty() {
        bail!("index url empty: pass --index or set index_url in runtime/settings.json");
    }

    let rt = PathBuf::from(&moddir).join("runtime");
    let dl = rt.join("download");
    let staging = rt.join("staging");
    let backup = rt.join("backup");
    fs::create_dir_all(&dl)?;
    fs::create_dir_all(&staging)?;
    fs::create_dir_all(&backup)?;

    // state.json（无论成功失败都尽量更新 last_run / last_error）
    let st_path = state_path(&moddir);
    let mut st = State::load_or_default(&st_path)?;
    st.touch_now();
    st.clear_error();

    if verbose {
        eprintln!(
            "[uxiconsd] update: moddir={} apply={} index={}",
            moddir, apply, index_url
        );
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent("uxiconsd/0.1")
        .build()?;

    // 1) 下载 index.json
    let idx: Index = client
        .get(&index_url)
        .send()
        .context("download index.json failed")?
        .error_for_status()
        .context("index.json http error")?
        .json()
        .context("parse index.json failed")?;

    // 2) channel 校验（用 settings 的 channel 作为期望）
    let expected_channel = cfg.channel.clone().unwrap_or("stable".to_string());
    if idx.channel != expected_channel {
        let msg = format!(
            "channel mismatch: index={} settings={}",
            idx.channel, expected_channel
        );
        st.set_error(msg.clone());
        st.save_pretty(&st_path)?;
        bail!(msg);
    }

    // 3) 仅检查模式
    if !apply {
        st.icons.channel = idx.channel.clone();
        st.icons.version = idx.icons_version.clone();
        st.icons.revision = idx.revision;
        st.icons.sha256 = idx.zip_sha256.clone();
        st.save_pretty(&st_path)?;
        return Ok(UpdateResult {
            checked: true,
            applied: false,
            message: "checked only (no apply)".to_string(),
            index_url,
            moddir,
            remote: Some(idx),
            downloaded_bytes: None,
            sha256_ok: None,
            size_ok: None,
        });
    }

    // 4) 下载 zip（清洗 URL，避免出现空白/换行）
    let download_url = idx.download_url.split_whitespace().collect::<String>();

    let zip_path = dl.join("icons.tmp.zip");
    let mut resp = client
        .get(&download_url)
        .send()
        .context("download zip failed")?
        .error_for_status()
        .context("zip http error")?;

    let mut f = fs::File::create(&zip_path)?;
    let mut downloaded: u64 = 0;

    loop {
        let mut buf = [0u8; 64 * 1024];
        let n = resp.read(&mut buf).context("read zip stream failed")?;
        if n == 0 {
            break;
        }
        f.write_all(&buf[..n])?;
        downloaded += n as u64;
    }
    f.flush()?;

    let size_ok = downloaded == idx.zip_size;
    if !size_ok {
        let msg = format!("size mismatch: got={} expect={}", downloaded, idx.zip_size);
        st.set_error(msg.clone());
        st.save_pretty(&st_path)?;
        bail!(msg);
    }

    // 5) sha256 校验
    let got = sha256_file(&zip_path)?;
    let sha_ok = got.eq_ignore_ascii_case(&idx.zip_sha256);
    if !sha_ok {
        let msg = "sha256 mismatch".to_string();
        st.set_error(msg.clone());
        st.save_pretty(&st_path)?;
        bail!(msg);
    }

    // 6) 解压到 staging/uxicons_unpack
    let staging_root = staging.join("uxicons_unpack");
    unzip_to(&zip_path, &staging_root).context("unzip failed")?;

    // 7) 应用到 bind mount 源目录：$MODDIR/uxicons/hdpi（不替换目录本体）
    apply_into_bound_hdpi(&moddir, &staging_root, &backup)
        .context("apply into bound hdpi failed")?;

    // 清理 staging（可选）
    let _ = fs::remove_dir_all(&staging_root);

    // 8) 更新 state.json
    st.icons.channel = idx.channel.clone();
    st.icons.version = idx.icons_version.clone();
    st.icons.revision = idx.revision;
    st.icons.sha256 = idx.zip_sha256.clone();
    st.icons.updated_at_utc = time::OffsetDateTime::now_utc().to_string();
    st.clear_error();
    st.save_pretty(&st_path)?;

    Ok(UpdateResult {
        checked: true,
        applied: true,
        message: "update applied".to_string(),
        index_url,
        moddir,
        remote: Some(idx),
        downloaded_bytes: Some(downloaded),
        sha256_ok: Some(true),
        size_ok: Some(true),
    })
}
