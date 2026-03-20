use crate::cli::scan::ScanCmd;

pub fn run(cmd: ScanCmd) -> anyhow::Result<()> {
    let path = cmd.target;

    if !path.exists() {
        anyhow::bail!("config not exists");
    }

    Ok(())
}
