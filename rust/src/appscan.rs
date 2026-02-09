use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use std::collections::HashSet;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ScanOptions {
    pub aapt2_path: String,
    pub user_only: bool,
    pub package: Option<String>,
    pub limit: Option<usize>,
    pub self_test: bool,
    pub verbose: bool,
}

#[derive(Debug, Serialize)]
pub struct ScanReport {
    pub aapt2: String,
    pub aapt2_check: String,
    pub total_packages: usize,
    pub scanned_packages: usize,
    pub apps: Vec<AppEntry>,
    pub note: String,
}

#[derive(Debug, Serialize)]
pub struct AppEntry {
    pub package: String,
    pub apk: String,
    pub themed_icon: Option<bool>,
    pub matched_xml: Option<String>,
    pub checked_xml_count: usize,
    pub reason: Option<String>,
}

pub fn scan_installed(opts: ScanOptions) -> Result<ScanReport> {
    if opts.verbose {
        eprintln!(
            "[uxiconsd] scan opts: user_only={} package={:?} limit={:?} self_test={}",
            opts.user_only, opts.package, opts.limit, opts.self_test
        );
        eprintln!("[uxiconsd] aapt2 path: {:?}", opts.aapt2_path);
    }

    let aapt2_msg = check_aapt2(&opts.aapt2_path, opts.verbose);

    if opts.self_test {
        let ok = aapt2_msg.is_ok();
        return Ok(ScanReport {
            aapt2: opts.aapt2_path,
            aapt2_check: match aapt2_msg {
                Ok(m) => m,
                Err(e) => format!("FAIL: {:#}", e),
            },
            total_packages: 0,
            scanned_packages: 0,
            apps: vec![AppEntry {
                package: "(self-test)".to_string(),
                apk: "".to_string(),
                themed_icon: Some(ok),
                matched_xml: None,
                checked_xml_count: 0,
                reason: Some(if ok {
                    "aapt2 self-test OK".to_string()
                } else {
                    "aapt2 self-test FAIL".to_string()
                }),
            }],
            note: "self-test mode".to_string(),
        });
    }

    let aapt2_check = aapt2_msg?;

    let mut pkgs = list_packages_with_apk().context("failed to list installed packages")?;
    let total = pkgs.len();

    if opts.user_only {
        pkgs.retain(|(_, apk)| apk.starts_with("/data/app/"));
    }
    if let Some(ref p) = opts.package {
        pkgs.retain(|(pkg, _)| pkg == p);
    }
    if let Some(n) = opts.limit && pkgs.len() > n {
        pkgs.truncate(n);
    }

    let mut out = Vec::with_capacity(pkgs.len());

    for (pkg, apk) in pkgs.iter() {
        match check_apk_themed_icon(&opts.aapt2_path, apk, opts.verbose) {
            Ok(CheckResult::Supported {
                matched_xml,
                checked,
            }) => out.push(AppEntry {
                package: pkg.clone(),
                apk: apk.clone(),
                themed_icon: Some(true),
                matched_xml: Some(matched_xml),
                checked_xml_count: checked,
                reason: Some("found monochrome layer in adaptive icon xml".to_string()),
            }),
            Ok(CheckResult::NotSupported { checked }) => out.push(AppEntry {
                package: pkg.clone(),
                apk: apk.clone(),
                themed_icon: Some(false),
                matched_xml: None,
                checked_xml_count: checked,
                reason: Some(
                    "no monochrome layer found under res/mipmap-anydpi-v26/*.xml".to_string(),
                ),
            }),
            Err(e) => out.push(AppEntry {
                package: pkg.clone(),
                apk: apk.clone(),
                themed_icon: None,
                matched_xml: None,
                checked_xml_count: 0,
                reason: Some(format!("scan error: {:#}", e)),
            }),
        }
    }

    Ok(ScanReport {
        aapt2: opts.aapt2_path,
        aapt2_check,
        total_packages: total,
        scanned_packages: out.len(),
        apps: out,
        note: "scan done".to_string(),
    })
}

fn list_packages_with_apk() -> Result<Vec<(String, String)>> {
    let output = Command::new("pm")
        .args(["list", "packages", "-f"])
        .output()
        .context("failed to run pm list packages -f")?;

    if !output.status.success() {
        return Err(anyhow!("pm returned non-zero: {:?}", output.status.code()));
    }

    let s = String::from_utf8_lossy(&output.stdout);
    let mut res = Vec::new();

    for line in s.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("package:") && let Some((path, pkg)) = rest.rsplit_once('=') {
            let pkg = pkg.trim().to_string();
            let path = path.trim().to_string();
            if !pkg.is_empty() && !path.is_empty() {
                res.push((pkg, path));
            }
        }
    }

    Ok(res)
}

fn check_aapt2(aapt2: &str, verbose: bool) -> Result<String> {
    if verbose {
        eprintln!("[uxiconsd] checking aapt2 with `version`: {:?}", aapt2);
    }

    let out = Command::new(aapt2)
        .arg("version")
        .output()
        .with_context(|| format!("failed to exec aapt2: {}", aapt2))?;

    if verbose {
        eprintln!("[uxiconsd] aapt2(version) exit={:?}", out.status.code());
        eprintln!(
            "[uxiconsd] aapt2(version) stdout: {}",
            String::from_utf8_lossy(&out.stdout)
        );
        eprintln!(
            "[uxiconsd] aapt2(version) stderr: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    if out.status.success() {
        // 一些版本把版本信息写到 stderr，stdout 可能为空
        let s1 = String::from_utf8_lossy(&out.stdout).trim().to_string();
        let s2 = String::from_utf8_lossy(&out.stderr).trim().to_string();
        if !s1.is_empty() {
            return Ok(s1);
        }
        if !s2.is_empty() {
            return Ok(s2);
        }
        return Ok("version ok".to_string());
    }

    if verbose {
        eprintln!("[uxiconsd] checking aapt2 with `dump --help` fallback");
    }
    let out2 = Command::new(aapt2)
        .args(["dump", "--help"])
        .output()
        .with_context(|| format!("failed to exec aapt2 (fallback): {}", aapt2))?;

    if verbose {
        eprintln!(
            "[uxiconsd] aapt2(dump --help) exit={:?}",
            out2.status.code()
        );
        eprintln!(
            "[uxiconsd] aapt2(dump --help) stderr: {}",
            String::from_utf8_lossy(&out2.stderr)
        );
    }

    if out2.status.success() {
        Ok("dump --help ok".to_string())
    } else {
        Err(anyhow!(
            "aapt2 not usable (version/help both failed), exit={:?}",
            out2.status.code()
        ))
    }
}

enum CheckResult {
    Supported { matched_xml: String, checked: usize },
    NotSupported { checked: usize },
}

fn check_apk_themed_icon(aapt2: &str, apk: &str, verbose: bool) -> Result<CheckResult> {
    let candidates = find_mipmap_anydpi_v26_xml(apk)?;
    if candidates.is_empty() {
        return Ok(CheckResult::NotSupported { checked: 0 });
    }

    let mut checked = 0usize;

    for xml in candidates {
        checked += 1;
        match aapt2_xmltree_has_monochrome(aapt2, apk, &xml, verbose) {
            Ok(true) => {
                return Ok(CheckResult::Supported {
                    matched_xml: xml,
                    checked,
                })
            }
            Ok(false) => continue,
            Err(e) => {
                // 如果 xmltree 对单个候选失败，继续尝试下一个；最终如果全失败，会在上层表现为 NotSupported，
                // 你也可以改成“全失败则 themed_icon=null”。当前保持可用。
                if verbose {
                    eprintln!("[uxiconsd] xmltree error (ignored): {:#}", e);
                }
                continue;
            }
        }
    }

    Ok(CheckResult::NotSupported { checked })
}

fn find_mipmap_anydpi_v26_xml(apk: &str) -> Result<Vec<String>> {
    let f = std::fs::File::open(apk).with_context(|| format!("open apk failed: {}", apk))?;
    let mut zip =
        zip::ZipArchive::new(f).with_context(|| format!("read apk as zip failed: {}", apk))?;

    let mut set = HashSet::new();
    for i in 0..zip.len() {
        let name = zip.by_index(i)?.name().to_string();
        if name.starts_with("res/mipmap-anydpi-v26/") && name.ends_with(".xml") {
            set.insert(name);
        }
    }

    let mut v: Vec<String> = set.into_iter().collect();
    v.sort();
    Ok(v)
}

fn aapt2_xmltree_has_monochrome(
    aapt2: &str,
    apk: &str,
    xml_in_apk: &str,
    verbose: bool,
) -> Result<bool> {
    if verbose {
        eprintln!(
            "[uxiconsd] aapt2 dump xmltree: apk={:?} xml={:?}",
            apk, xml_in_apk
        );
    }

    // 兼容你当前 aapt2：需要 --file
    let output = Command::new(aapt2)
        .args(["dump", "xmltree", apk, "--file", xml_in_apk])
        .output()
        .with_context(|| format!("run aapt2 dump xmltree failed: {} {}", apk, xml_in_apk))?;

    if verbose && !output.status.success() {
        eprintln!(
            "[uxiconsd] aapt2 xmltree failed exit={:?}",
            output.status.code()
        );
        eprintln!(
            "[uxiconsd] stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    Ok(text.contains("monochrome"))
}
