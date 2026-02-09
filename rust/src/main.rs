mod appscan;
mod state;
mod update;

use anyhow::Result;
use clap::{Parser, Subcommand};

const DEFAULT_MODULE_ID: &str = "ColorOSIconsPatch";
const DEFAULT_AAPT2_PATH: &str = "/data/adb/modules/ColorOSIconsPatch/bin/aapt2";

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
        /// Module dir (if set, aapt2 defaults to <moddir>/bin/aapt2)
        #[arg(long)]
        moddir: Option<String>,

        /// Path to aapt2 binary (default: module bin/aapt2)
        #[arg(long, default_value = DEFAULT_AAPT2_PATH)]
        aapt2: String,

        /// Only scan 3rd-party apps (/data/app). (Best-effort filter)
        #[arg(long, default_value_t = false)]
        user_only: bool,

        /// Only scan a single package (debug)
        #[arg(long)]
        package: Option<String>,

        /// Limit number of packages scanned (debug)
        #[arg(long)]
        limit: Option<usize>,

        /// Run aapt2 self-test (version/help) and exit
        #[arg(long, default_value_t = false)]
        self_test: bool,

        /// Verbose logs to stderr (stdout stays JSON)
        #[arg(long, default_value_t = false)]
        verbose: bool,
    },

    /// Online update icons data using index.json + icons.zip (skeleton)
    Update {
        /// index.json URL
        #[arg(long)]
        index: String,

        /// Module dir (e.g. /data/adb/modules/<id>)
        #[arg(long)]
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
            // fill defaults (module info best-effort)
            st.module.id = DEFAULT_MODULE_ID.to_string();
            st.touch_now();
            st.save_pretty(&state)?;
            println!("{}", serde_json::to_string_pretty(&st)?);
        }

        Cmd::Scan {
            moddir,
            mut aapt2,
            user_only,
            package,
            limit,
            self_test,
            verbose,
        } => {
            // If moddir provided, prefer <moddir>/bin/aapt2 unless user explicitly set --aapt2
            if let Some(md) = moddir.as_deref() {
                // If user didn't override (still default), rewrite to moddir-based
                if aapt2 == DEFAULT_AAPT2_PATH {
                    aapt2 = format!("{}/bin/aapt2", md.trim_end_matches('/'));
                }
            }

            let opts = appscan::ScanOptions {
                aapt2_path: aapt2,
                user_only,
                package,
                limit,
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
            let out = update::run_update(&index, &moddir, apply, verbose)?;
            println!("{}", serde_json::to_string_pretty(&out)?);
        }
    }

    Ok(())
}
