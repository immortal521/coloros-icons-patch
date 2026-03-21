use crate::cli::config::{GetCmd, SetCmd};
use crate::config::loader;
use anyhow::Result;
use std::fs;
use toml_edit::{value, DocumentMut};

/// 读取配置
pub fn get(cmd: GetCmd) -> Result<()> {
    let config = loader::load(cmd.config)?;

    if cmd.json {
        println!("{}", serde_json::to_string_pretty(&config)?);
    } else {
        println!("{:#?}", config);
    }

    Ok(())
}

/// 修改配置
pub fn set(cmd: SetCmd) -> Result<()> {
    let path = cmd.config;

    let content = fs::read_to_string(&path)?;
    let mut doc = content.parse::<DocumentMut>()?;

    if let Some(channel) = cmd.channel {
        doc["default"]["channel"] = value(channel);
    }

    if let Some(target) = cmd.target_dir {
        doc["default"]["target_dir"] = value(target.to_str().unwrap());
    }

    if let Some(temp) = cmd.temp_dir {
        doc["default"]["temp_dir"] = value(temp.to_str().unwrap());
    }

    if let Some(version) = cmd.icons_version {
        doc["default"]["icons_version"] = value(version);
    }

    fs::write(&path, doc.to_string())?;

    println!("✔ config updated");

    Ok(())
}
