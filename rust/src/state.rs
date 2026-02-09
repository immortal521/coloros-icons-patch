use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub module: ModuleState,
    pub icons: IconsState,
    pub last_run_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleState {
    pub id: String,
    pub version: String,
    pub version_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconsState {
    pub channel: String,
    pub version: String,
    pub revision: i64,
    pub sha256: String,
    pub updated_at_utc: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            module: ModuleState {
                id: "".to_string(),
                version: "0.0.0".to_string(),
                version_code: 0,
            },
            icons: IconsState {
                channel: "stable".to_string(),
                version: "".to_string(),
                revision: 0,
                sha256: "".to_string(),
                updated_at_utc: "".to_string(),
            },
            last_run_utc: None,
        }
    }
}

impl State {
    pub fn load_or_default(path: &str) -> Result<Self> {
        if !Path::new(path).exists() {
            return Ok(Self::default());
        }
        let data = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save_pretty(&self, path: &str) -> Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn touch_now(&mut self) {
        self.last_run_utc = Some(OffsetDateTime::now_utc().to_string());
    }
}
