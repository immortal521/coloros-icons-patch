use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub module_id: Option<String>,
    pub channel: Option<String>,
    pub index_url: Option<String>,
    pub aapt2_path: Option<String>,
    pub scan_user_only: Option<bool>,
    pub scan_limit: Option<usize>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            module_id: Some("ColorOSIconsPatch".to_string()),
            channel: Some("stable".to_string()),
            index_url: None,
            aapt2_path: None,
            scan_user_only: Some(true),
            scan_limit: None,
        }
    }
}

impl Settings {
    pub fn load_or_default(path: &str) -> Result<Self> {
        if !Path::new(path).exists() {
            return Ok(Self::default());
        }
        let data = fs::read_to_string(path)?;
        let mut v: Settings = serde_json::from_str(&data)?;
        let d = Settings::default();

        if v.module_id.is_none() {
            v.module_id = d.module_id;
        }
        if v.channel.is_none() {
            v.channel = d.channel;
        }
        if v.scan_user_only.is_none() {
            v.scan_user_only = d.scan_user_only;
        }

        Ok(v)
    }
}
