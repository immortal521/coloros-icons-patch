use serde::Serialize;

use crate::{
    cli::check::CheckCmd,
    config::model::{Config, Index},
};

#[derive(Serialize)]
struct CheckResult {
    current_version: String,
    latest_version: String,
    update_url: String,
    checksum: String,
    has_update: bool,
    update_name: String,
    update_size: u64,
    published_at: String,
    notes: String,
    revision: u32,
}

pub async fn run(cmd: CheckCmd) -> anyhow::Result<()> {
    let path = cmd.config;

    if !path.exists() {
        anyhow::bail!("config not exists: {:?}", path);
    }

    let content = std::fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&content)?;

    let channel = &config.default.channel;

    let source = config
        .sources
        .get(channel)
        .ok_or_else(|| anyhow::anyhow!("source '{}' not found", channel))?;

    let index: Index = reqwest::get(&source.url).await?.json().await?;

    let current_version = config.default.icons_version.clone();
    let has_update = current_version != index.icons_version;

    let result = CheckResult {
        current_version,
        latest_version: index.icons_version,
        update_url: index.download_url,
        checksum: index.zip_sha256,
        has_update,
        update_name: index.zip_name,
        update_size: index.zip_size,
        published_at: index.published_at,
        notes: index.notes,
        revision: index.revision,
    };

    if cmd.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("Current version: {}", result.current_version);
        println!("Latest version: {}", result.latest_version);
        println!("Update URL: {}", result.update_url);
        println!("Checksum: {}", result.checksum);
        println!("Has update: {}", result.has_update);
    }

    Ok(())
}
