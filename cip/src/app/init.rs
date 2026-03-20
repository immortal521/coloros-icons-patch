use crate::cli::init::InitCmd;
use std::fs;
use toml_edit::{value, DocumentMut};

const DEFAULT_CONFIG: &str = r#"[default]
icons_version = "0.1.0"
channel = "stable"
runtime_dir = "./runtime"
temp_dir = "/tmp/cip"
target_dir = "./icons"

[sources.beta]
url = "https://immortal521.github.io/coloros-icons-patch/beta/index.json"

[sources.stable]
url = "https://immortal521.github.io/coloros-icons-patch/stable/index.json"
"#;

pub fn run(cmd: InitCmd) -> anyhow::Result<()> {
    let path = cmd.config;

    if path.exists() && !cmd.force {
        anyhow::bail!("config exists");
    }

    if let Some(p) = path.parent() {
        fs::create_dir_all(p)?;
    }

    let mut doc = DEFAULT_CONFIG.parse::<DocumentMut>()?;

    doc["default"]["channel"] = value(cmd.channel);

    if let Some(temp_dir) = cmd.temp_dir {
        doc["default"]["temp_dir"] = value(temp_dir.to_str().unwrap());
    }

    if let Some(target_dir) = cmd.target_dir {
        doc["default"]["target_dir"] = value(target_dir.to_str().unwrap());
    }

    fs::write(&path, doc.to_string())?;
    println!("✔ config created at {:?}", path);

    Ok(())
}
