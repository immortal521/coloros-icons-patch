use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UpdateResult {
    pub checked: bool,
    pub applied: bool,
    pub message: String,
    pub index_url: String,
    pub moddir: String,
}

pub fn run_update(
    index_url: &str,
    moddir: &str,
    apply: bool,
    verbose: bool,
) -> Result<UpdateResult> {
    if verbose {
        eprintln!(
            "[uxiconsd] update: index={} moddir={} apply={}",
            index_url, moddir, apply
        );
    }

    Ok(UpdateResult {
        checked: true,
        applied: apply,
        message: "placeholder: update not implemented yet".to_string(),
        index_url: index_url.to_string(),
        moddir: moddir.to_string(),
    })
}
