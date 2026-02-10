mod appscan;
mod settings;
mod state;
mod update;

use anyhow::Result;
use clap::{Parser, Subcommand};

const DEFAULT_MODULE_ID: &str = "ColorOSIconsPatch";
const DEFAULT_MODDIR: &str = "/data/adb/modules/ColorOSIconsPatch";
const DEFAULT_AAPT2_PATH: &str = "/data/adb/modules/ColorOSIconsPatch/bin/aapt2";

fn settings_path(moddir: &str) -> String {
    format!("{}/runtime/settings.json", moddir.trim_end_matches('/'))
}
fn state_path(moddir: &str) -> String {
    format!("{}/runtime/state.json", moddir.trim_end_matches('/'))
}

#[derive(Parser)]
#[command(name = "uxiconsd")]
#[command(about = "ColorOS Icons Patch (KernelSU module)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Read/write runtime state.json (for WebUI and debugging)
    Status {
        /// Path to state.json (e.g. /data/adb/modules/<id>/runtime/state.json)
        #[arg(long)]
        state: String,
    },

    /// Scan installed apps and check themed icon support using aapt2
    Scan {
        /// Module dir (default: /data/adb/modules/ColorOSIconsPatch)
        #[arg(long, default_value = DEFAULT_MODDIR)]
        moddir: String,

        /// Path to aapt2 binary (if not set, read from settings.json, else fallback to <moddir>/bin/aapt2)
        #[arg(long)]
        aapt2: Option<String>,

        /// Only scan 3rd-party apps (/data/app). (If not set, read from settings.json)
        #[arg(long, default_value_t = false)]
        user_only: bool,

        /// Force scan all apps (override settings/user_only)
        #[arg(long, default_value_t = false)]
        all: bool,

        /// Only scan a single package (debug)
        #[arg(long)]
        package: Option<String>,

        /// Limit number of packages scanned (debug). If not set, read from settings.json.
        #[arg(long)]
        limit: Option<usize>,

        /// Run aapt2 self-test (version/help) and exit
        #[arg(long, default_value_t = false)]
        self_test: bool,

        /// Verbose logs to stderr (stdout stays JSON)
        #[arg(long, default_value_t = false)]
        verbose: bool,
    },

    /// Online update icons data using index.json + icons.zip
    Update {
        /// index.json URL (optional: if missing, read from settings.json)
        #[arg(long)]
        index: Option<String>,

        /// Module dir (default: /data/adb/modules/ColorOSIconsPatch)
        #[arg(long, default_value = DEFAULT_MODDIR)]
        moddir: String,

        /// Apply update (download + verify + replace). If false, only check.
        #[arg(long, default_value_t = false)]
        apply: bool,

        /// Verbose logs to stderr
        #[arg(long, default_value_t = false)]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Status { state } => {
            let mut st = state::State::load_or_default(&state)?;
            st.module.id = DEFAULT_MODULE_ID.to_string();
            st.touch_now();
            st.save_pretty(&state)?;
            println!("{}", serde_json::to_string_pretty(&st)?);
        }

        Cmd::Scan {
            moddir,
            aapt2,
            user_only,
            all,
            package,
            limit,
            self_test,
            verbose,
        } => {
            let cfg = settings::Settings::load_or_default(&settings_path(&moddir))?;

            // aapt2 优先级：CLI --aapt2 > settings.json aapt2_path > <moddir>/bin/aapt2 > DEFAULT_AAPT2_PATH
            let aapt2_path = if let Some(p) = aapt2 {
                p
            } else if let Some(p) = cfg.aapt2_path.clone() {
                p
            } else {
                let guess = format!("{}/bin/aapt2", moddir.trim_end_matches('/'));
                if std::path::Path::new(&guess).exists() {
                    guess
                } else {
                    DEFAULT_AAPT2_PATH.to_string()
                }
            };

            // user_only 优先级：
            // --all 强制 false
            // --user-only 强制 true
            // 否则用 settings.scan_user_only（默认 true）
            let user_only_final = if all {
                false
            } else if user_only {
                true
            } else {
                cfg.scan_user_only.unwrap_or(true)
            };

            // limit：CLI > settings
            let limit_final = limit.or(cfg.scan_limit);

            let opts = appscan::ScanOptions {
                aapt2_path,
                user_only: user_only_final,
                package,
                limit: limit_final,
                self_test,
                verbose,
            };

            let report = appscan::scan_installed(opts)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }

        Cmd::Update {
            index,
            moddir,
            apply,
            verbose,
        } => {
            let idx = index.as_deref();
            let out = update::run_update(idx, &moddir, apply, verbose)?;
            println!("{}", serde_json::to_string_pretty(&out)?);
        }
    }

    Ok(())
}
