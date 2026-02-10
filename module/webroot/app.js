import { exec } from "./vendor/kernelsu.js";

function $(id) {
  return document.getElementById(id);
}

function nowIso() {
  try {
    return new Date().toISOString();
  } catch {
    return "";
  }
}

function toast(msg) {
  const el = $("toast");
  if (!el) return;
  el.textContent = msg;
  el.classList.add("show");
  setTimeout(() => el.classList.remove("show"), 2200);
}

function escSingleQuotes(s) {
  return String(s).replace(/'/g, `'\\''`);
}

function setHint(id, msg, isErr = false) {
  const el = $(id);
  if (!el) return;
  el.textContent = msg || "";
  el.classList.toggle("error", !!isErr);
}

function setButtonsDisabled(disabled) {
  const ids = [
    "btn-selftest",
    "btn-scan",
    "btn-update",
    "btn-save",
    "btn-load",
  ];
  for (const id of ids) {
    const el = $(id);
    if (el) el.disabled = disabled;
  }
}

// 防止大对象 stringify 卡死：不 pretty；scan apps 超过阈值裁剪
function setResult(obj) {
  const el = $("result-json");
  if (!el) return;

  try {
    if (obj && Array.isArray(obj.apps) && obj.apps.length > 200) {
      const total = obj.apps.length;
      const clipped = {
        ...obj,
        apps: obj.apps.slice(0, 200),
        note: `${obj.note || ""} (WebUI仅显示前 200 / 总 ${total})`.trim(),
      };
      el.textContent = JSON.stringify(clipped);
      return;
    }
    el.textContent = JSON.stringify(obj);
  } catch (e) {
    el.textContent = `render error: ${String(e?.message || e)}`;
  }
}

// --- exec/sh：强制返回 string，兼容不同 bridge 字段名 ---
async function sh(cmd, { timeoutMs = 60000 } = {}) {
  const p = exec(`sh -c '${escSingleQuotes(cmd)}'`);
  const t = new Promise((_, rej) =>
    setTimeout(() => rej(new Error("timeout")), timeoutMs),
  );

  const res = await Promise.race([p, t]);

  if (typeof res === "string") return res;

  const errno = res?.errno ?? res?.code ?? res?.status ?? 0;

  const stdout =
    res?.stdout != null
      ? String(res.stdout)
      : res?.out != null
        ? String(res.out)
        : res?.output != null
          ? String(res.output)
          : res?.result != null
            ? String(res.result)
            : "";

  const stderr =
    res?.stderr != null
      ? String(res.stderr)
      : res?.err != null
        ? String(res.err)
        : "";

  if (errno !== 0) {
    const msg = stderr.trim() || stdout.trim() || `errno=${errno}`;
    throw new Error(msg);
  }
  return stdout;
}

// --- Settings ---
function getDefaultModdir() {
  return "/data/adb/modules/ColorOSIconsPatch";
}

function getSettingsFromUI() {
  const moddir = $("in-moddir")?.value.trim() || getDefaultModdir();
  const channel = $("in-channel")?.value || "stable";
  const index = $("in-index")?.value.trim() || "";
  const aapt2 = $("in-aapt2")?.value.trim() || `${moddir}/bin/aapt2`;
  const userOnly = !!$("in-useronly")?.checked;
  const limitRaw = $("in-limit")?.value.trim() || "";
  const limit = limitRaw ? Number(limitRaw) : null;

  return {
    module_id: "ColorOSIconsPatch",
    channel,
    index_url: index,
    aapt2_path: aapt2,
    scan_user_only: userOnly,
    scan_limit: Number.isFinite(limit) ? limit : null,
    moddir,
  };
}

function saveLocal(settings) {
  localStorage.setItem("ux_moddir", settings.moddir);
  localStorage.setItem("ux_channel", settings.channel);
  localStorage.setItem("ux_index", settings.index_url);
  localStorage.setItem("ux_aapt2", settings.aapt2_path);
  localStorage.setItem("ux_useronly", settings.scan_user_only ? "1" : "0");
  localStorage.setItem(
    "ux_limit",
    settings.scan_limit == null ? "" : String(settings.scan_limit),
  );
}

function loadLocalToUI() {
  const moddir = localStorage.getItem("ux_moddir") || getDefaultModdir();
  const channel = localStorage.getItem("ux_channel") || "stable";

  $("in-moddir").value = moddir;
  $("in-channel").value = channel;

  $("in-index").value =
    localStorage.getItem("ux_index") ||
    `https://immortal521.github.io/coloros-icons-patch/${channel}/index.json`;

  $("in-aapt2").value =
    localStorage.getItem("ux_aapt2") || `${moddir}/bin/aapt2`;

  $("in-useronly").checked =
    (localStorage.getItem("ux_useronly") || "1") === "1";
  $("in-limit").value = localStorage.getItem("ux_limit") || "";
}

async function writeSettingsFile(settings) {
  const moddir = settings.moddir;
  // 不用 pretty，避免多行导致 bridge 截断（虽然我们读用 base64 了，但写也尽量简单）
  const json = JSON.stringify({
    module_id: settings.module_id,
    channel: settings.channel,
    index_url: settings.index_url,
    aapt2_path: settings.aapt2_path,
    scan_user_only: settings.scan_user_only,
    scan_limit: settings.scan_limit,
  });

  const cmd = `
    mkdir -p '${moddir}/runtime' &&
    cat > '${moddir}/runtime/settings.json' <<'EOF'
${json}
EOF
  `;
  await sh(cmd);
}

async function readSettingsFile(moddir) {
  const p = `${moddir}/runtime/settings.json`;
  const b64 = String(
    await sh(`base64 '${p}' 2>/dev/null | tr -d '\n' || true`),
  ).trim();
  if (!b64) return null;

  let txt = atob(b64);
  txt = txt.replace(/^\uFEFF/, "").trim();
  return JSON.parse(txt);
}

// --- uxiconsd runner with persistent logs ---
async function uxiconsdJson(args, { timeoutMs = 60000, tag = "cmd" } = {}) {
  const moddir = $("in-moddir")?.value.trim() || getDefaultModdir();

  const rt = `${moddir}/runtime`;
  const outFile = `${rt}/webui_last.json`;
  const errFile = `${rt}/webui_last.err`;
  const metaFile = `${rt}/webui_last.meta.json`;

  const started = Date.now();
  const stamp = nowIso();

  // 注意：
  // - 二进制在 bin/uxiconsd
  // - stdout/stderr 分离保存
  // - meta 记录命令与耗时
  // - 只把 outFile base64 输出回 WebUI（单行）
  const cmd = `
    mkdir -p '${rt}' &&
    cd '${moddir}' &&
    echo '{ "ts": "${stamp}", "tag": "${tag}", "cmd": "./bin/uxiconsd ${escSingleQuotes(args)}" }' > '${metaFile}' &&
    ./bin/uxiconsd ${args} > '${outFile}' 2> '${errFile}' ;
    ret=$? ;
    dur_ms=$(( $(date +%s%3N 2>/dev/null || echo 0) - $(date +%s%3N 2>/dev/null || echo 0) )) ;
    # 追加更多 meta（不依赖 date+%s%3N 存在）
    echo '{ "ts": "${stamp}", "tag": "${tag}", "ret": '$ret', "elapsed_ms": ${Date.now()} }' > '${metaFile}' ;
    if [ $ret -ne 0 ]; then
      # 输出错误摘要一行，供 sh() 抛错展示
      head -c 2000 '${errFile}' | tr '\\n' ' ' ;
      exit $ret ;
    fi ;
    base64 '${outFile}' | tr -d '\\n'
  `;

  let b64;
  try {
    b64 = String(await sh(cmd, { timeoutMs })).trim();
  } catch (e) {
    // 失败时：把 runtime 里的 err/meta 尽量读回显示到 DOM
    const errTxt = String(
      await sh(`cat '${errFile}' 2>/dev/null | head -c 4000 || true`),
    ).trim();
    const metaTxt = String(
      await sh(`cat '${metaFile}' 2>/dev/null | head -c 2000 || true`),
    ).trim();
    const msg = `uxiconsd 执行失败：${String(e?.message || e)}\n\n[meta]\n${metaTxt}\n\n[stderr]\n${errTxt}`;
    throw new Error(msg);
  }

  if (!b64) {
    const errTxt = String(
      await sh(`cat '${errFile}' 2>/dev/null | head -c 4000 || true`),
    ).trim();
    throw new Error(`uxiconsd 输出为空\n\n[stderr]\n${errTxt}`);
  }

  let txt;
  try {
    txt = atob(b64);
  } catch {
    const head = b64.slice(0, 120);
    throw new Error(`输出不是 base64（可能被 bridge 截断），head="${head}"`);
  }

  txt = txt.replace(/^\uFEFF/, "").trim();

  // 解析失败：回读一份 outFile 头部帮助定位
  try {
    return JSON.parse(txt);
  } catch (e) {
    const outHead = txt.slice(0, 400).replace(/\n/g, "\\n");
    const errTxt = String(
      await sh(`cat '${errFile}' 2>/dev/null | head -c 2000 || true`),
    ).trim();
    throw new Error(
      `JSON 解析失败：${e.message}\n\n[stdout head]\n${outHead}\n\n[stderr]\n${errTxt}`,
    );
  } finally {
    const elapsed = Date.now() - started;
    // 给用户一点反馈（不阻塞）
    console.log(`[uxiconsdJson] ${tag} elapsed=${elapsed}ms`);
  }
}

// --- UI render ---
function renderState(st) {
  $("st-module").textContent =
    `${st?.module?.id || "-"} ${st?.module?.version || ""}`.trim();
  $("st-channel").textContent = st?.icons?.channel || "-";
  $("st-icons-version").textContent = st?.icons?.version || "-";
  $("st-icons-rev").textContent = String(st?.icons?.revision ?? "-");
  $("st-updated-at").textContent = st?.icons?.updated_at_utc || "-";
  $("st-last-run").textContent = st?.last_run_utc || "-";

  const errEl = $("st-last-error");
  if (!errEl) return;

  if (st?.last_error) {
    errEl.textContent = `错误：${st.last_error}`;
    errEl.classList.add("error");
  } else {
    errEl.textContent = "";
    errEl.classList.remove("error");
  }
}

async function refreshStatus() {
  const moddir = $("in-moddir")?.value.trim() || getDefaultModdir();
  const statePath = `${moddir}/runtime/state.json`;
  const st = await uxiconsdJson(`status --state '${statePath}'`, {
    timeoutMs: 20000,
    tag: "status",
  });
  renderState(st);
  setResult(st);
}

// --- nav ---
function setupNav() {
  const buttons = Array.from(document.querySelectorAll(".nav-item"));
  const pages = {
    home: document.getElementById("page-home"),
    settings: document.getElementById("page-settings"),
  };

  buttons.forEach((btn) => {
    btn.addEventListener("click", () => {
      const target = btn.dataset.target;
      buttons.forEach((b) => b.classList.toggle("active", b === btn));
      Object.entries(pages).forEach(([k, el]) =>
        el.classList.toggle("active", k === target),
      );
    });
  });
}

// --- actions ---
async function onSaveSettings() {
  try {
    setButtonsDisabled(true);
    const s = getSettingsFromUI();
    saveLocal(s);
    await writeSettingsFile(s);
    setHint(
      "settings-hint",
      "已保存：localStorage + runtime/settings.json",
      false,
    );
    toast("设置已保存");
  } catch (e) {
    setHint("settings-hint", `保存失败：${String(e?.message || e)}`, true);
  } finally {
    setButtonsDisabled(false);
  }
}

async function onLoadFromFile() {
  try {
    setButtonsDisabled(true);
    const moddir = $("in-moddir")?.value.trim() || getDefaultModdir();
    const file = await readSettingsFile(moddir);
    if (!file) {
      setHint("settings-hint", "未找到 settings.json（请先保存）", true);
      return;
    }

    $("in-channel").value = file.channel || "stable";
    $("in-index").value = file.index_url || "";
    $("in-aapt2").value = file.aapt2_path || `${moddir}/bin/aapt2`;
    $("in-useronly").checked = file.scan_user_only !== false;
    $("in-limit").value =
      file.scan_limit == null ? "" : String(file.scan_limit);

    const merged = getSettingsFromUI();
    saveLocal(merged);

    setHint(
      "settings-hint",
      "已从 runtime/settings.json 读取并同步到本地",
      false,
    );
    toast("已读取 settings.json");
  } catch (e) {
    setHint("settings-hint", `读取失败：${String(e?.message || e)}`, true);
  } finally {
    setButtonsDisabled(false);
  }
}

async function onSelfTest() {
  try {
    setButtonsDisabled(true);
    setHint("op-hint", "正在自检…", false);

    const moddir = $("in-moddir")?.value.trim() || getDefaultModdir();
    const obj = await uxiconsdJson(
      `scan --moddir '${moddir}' --self-test --verbose`,
      { timeoutMs: 30000, tag: "selftest" },
    );

    setResult(obj);
    setHint("op-hint", "自检完成", false);
  } catch (e) {
    const msg = String(e?.message || e);
    setHint("op-hint", "自检失败（详见结果区与 runtime/webui_last.err）", true);
    setResult({ error: msg });
  } finally {
    setButtonsDisabled(false);
  }
}

async function onScan() {
  try {
    setButtonsDisabled(true);
    setHint("op-hint", "正在扫描…（可能较久）", false);

    const moddir = $("in-moddir")?.value.trim() || getDefaultModdir();
    const limitRaw = $("in-limit")?.value.trim() || "";
    const limit = limitRaw ? Number(limitRaw) : null;
    const limitArg = Number.isFinite(limit) ? ` --limit ${limit}` : "";

    const obj = await uxiconsdJson(
      `scan --moddir '${moddir}' --verbose${limitArg}`,
      { timeoutMs: 180000, tag: "scan" },
    );

    setResult(obj);
    setHint(
      "op-hint",
      `扫描完成：${obj.scanned_packages}/${obj.total_packages}`,
      false,
    );
  } catch (e) {
    const msg = String(e?.message || e);
    setHint("op-hint", "扫描失败（详见结果区与 runtime/webui_last.err）", true);
    setResult({ error: msg });
  } finally {
    setButtonsDisabled(false);
  }
}

async function onUpdate() {
  try {
    setButtonsDisabled(true);
    setHint("op-hint", "正在更新…", false);

    const moddir = $("in-moddir")?.value.trim() || getDefaultModdir();
    const obj = await uxiconsdJson(
      `update --moddir '${moddir}' --apply --verbose`,
      { timeoutMs: 180000, tag: "update" },
    );

    setResult(obj);
    setHint("op-hint", obj.message || "更新完成", false);
    await refreshStatus();
  } catch (e) {
    const msg = String(e?.message || e);
    setHint("op-hint", "更新失败（详见结果区与 runtime/webui_last.err）", true);
    setResult({ error: msg });
    try {
      await refreshStatus();
    } catch {}
  } finally {
    setButtonsDisabled(false);
  }
}

// --- boot ---
document.addEventListener("DOMContentLoaded", async () => {
  setupNav();
  loadLocalToUI();

  $("btn-save")?.addEventListener("click", onSaveSettings);
  $("btn-load")?.addEventListener("click", onLoadFromFile);
  $("btn-selftest")?.addEventListener("click", onSelfTest);
  $("btn-scan")?.addEventListener("click", onScan);
  $("btn-update")?.addEventListener("click", onUpdate);

  // 初次进入：尝试从文件同步，然后刷新状态
  try {
    await onLoadFromFile();
  } catch {}
  try {
    await refreshStatus();
  } catch (e) {
    setHint("op-hint", `状态读取失败：${String(e?.message || e)}`, true);
    setResult({ error: String(e?.message || e) });
  }
});
