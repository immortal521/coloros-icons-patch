use anyhow::Result;

use crate::{cli::update::UpdateCmd, config::loader, core::updater};

pub async fn run(cmd: UpdateCmd) -> Result<()> {
    let config = loader::load(cmd.config)?;

    updater::run(config, cmd.download_base).await
}
