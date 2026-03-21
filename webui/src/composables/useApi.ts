import { PATHS } from "../constants";
import type { Config } from "../types/config";

/* ---------------- KernelSU Types ---------------- */

interface ExecOptions {
  cwd?: string;
  env?: Record<string, string>;
}

interface ExecResult {
  errno: number;
  stdout: string;
  stderr: string;
}

type KsuExec = (cmd: string, options?: ExecOptions) => Promise<ExecResult>;

interface SpawnOptions {
  cwd?: string;
  env?: Record<string, string>;
}

interface Stdio {
  on(event: "data", callback: (data: string) => void): void;
}

interface ChildProcess {
  stdout: Stdio;
  stderr: Stdio;
  on(event: "exit", callback: (code: number) => void): void;
  on(event: "error", callback: (err: any) => void): void;
}

type KsuSpawn = (command: string, args?: string[], options?: SpawnOptions) => ChildProcess;

/* ---------------- Update Types ---------------- */

interface UpdateInfo {
  current_version: string;
  latest_version: string;
  update_url: string;
  checksum: string;
  has_update: boolean;
  update_name: string;
  update_size: number;
  published_at: string;
  notes: string;
  revision: number;
}

type UpdateEvent =
  | { type: "stage"; value: string }
  | { type: "progress"; stage: string; value: number }
  | { type: "info"; message?: string }
  | { type: "done" }
  | { type: "log"; message: string }
  | { type: "error"; message: string };

/* ---------------- KernelSU Runtime ---------------- */

let ksuExec: KsuExec | null = null;
let ksuSpawn: KsuSpawn | null = null;
let initPromise: Promise<void> | null = null;

const initKernelSU = async () => {
  if (ksuExec && ksuSpawn) return;

  if (!initPromise) {
    initPromise = (async () => {
      try {
        const mod = (await import("kernelsu")) as any;

        // ✅ 兼容 default export
        const api = mod.default ?? mod;

        ksuExec = api.exec ?? null;
        ksuSpawn = api.spawn ?? null;

        console.log("[KernelSU] loaded:", {
          exec: !!ksuExec,
          spawn: !!ksuSpawn,
        });
      } catch (e) {
        console.error("[KernelSU] init failed ❌", e);
        ksuExec = null;
        ksuSpawn = null;
      }
    })();
  }

  await initPromise;
};

/* ---------------- API ---------------- */

export function useAPI() {
  /* ---------- ensure ---------- */

  const ensureExec = async (): Promise<KsuExec> => {
    await initKernelSU();
    if (!ksuExec) throw new Error("KernelSU exec not available");
    return ksuExec;
  };

  const ensureSpawn = async (): Promise<KsuSpawn> => {
    await initKernelSU();
    if (!ksuSpawn) throw new Error("KernelSU spawn not available");
    return ksuSpawn;
  };

  /* ---------- exec helper ---------- */

  const execOrThrow = async (cmd: string) => {
    const exec = await ensureExec();
    const res = await exec(cmd);

    if (res.errno !== 0) {
      throw new Error(res.stderr?.trim() || `Command failed: ${cmd}`);
    }

    return res.stdout.trim();
  };

  /* ---------- config ---------- */

  const loadConfig = async (): Promise<Config> => {
    const stdout = await execOrThrow(`${PATHS.CIP_BIN} config get --config ${PATHS.CONFIG} --json`);
    return JSON.parse(stdout);
  };

  const setChannel = async (channel: string) => {
    await execOrThrow(`${PATHS.CIP_BIN} config set --config ${PATHS.CONFIG} --channel ${channel}`);
  };

  const setIconsVersion = async (version: string) => {
    await execOrThrow(
      `${PATHS.CIP_BIN} config set --config ${PATHS.CONFIG} --icons_version ${version}`,
    );
  };

  /* ---------- check update ---------- */

  const checkUpdate = async (): Promise<UpdateInfo> => {
    const stdout = await execOrThrow(`${PATHS.CIP_BIN} check --config ${PATHS.CONFIG} --json`);
    return JSON.parse(stdout);
  };

  /* ---------- 🚀 流式 update ---------- */

  const updateStream = async (
    onEvent: (e: UpdateEvent) => void,
    onDone?: () => void,
    onError?: (err: string) => void,
  ) => {
    const spawn = await ensureSpawn();

    const cmd = PATHS.CIP_BIN;
    const args = ["update", "--config", PATHS.CONFIG, "--json"];

    console.log("[update] spawn:", cmd, args);

    onEvent({ type: "log", message: "启动更新进程" });

    const child = spawn(cmd, args);
    let newVersion: string | null = null;

    let buffer = "";

    /* ---------- stdout ---------- */
    child.stdout.on("data", (chunk: string) => {
      console.log("[stdout raw]", chunk);

      buffer += chunk;

      const lines = buffer.split("\n");
      buffer = lines.pop() || "";

      for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed) continue;

        try {
          const parsed = JSON.parse(trimmed);

          if (parsed.type === "info" && parsed.value === "version") {
            newVersion = parsed.version;
            console.log("[update] captured version:", newVersion);
          }

          console.log("[stdout json]", parsed);
          onEvent(parsed);
        } catch {
          // 非 JSON → 当日志处理
          console.warn("[stdout text]", trimmed);

          onEvent({
            type: "log",
            message: trimmed,
          });
        }
      }
    });

    /* ---------- stderr ---------- */
    child.stderr.on("data", (err: string) => {
      console.error("[stderr]", err);

      onEvent({
        type: "error",
        message: err.trim(),
      });

      onError?.(err);
    });

    /* ---------- exit ---------- */
    child.on("exit", async (code: number) => {
      console.log("[update] exit:", code);

      if (code === 0) {
        onEvent({ type: "done" });
        if (newVersion) {
          await setIconsVersion(newVersion);
        } else {
          const info = await checkUpdate();
          await setIconsVersion(info.latest_version);
        }
        onDone?.();
        return;
      }

      const msg = `exit code ${code}`;
      onEvent({ type: "error", message: msg });
      onError?.(msg);
    });

    /* ---------- error ---------- */
    child.on("error", (err: any) => {
      console.error("[update] spawn error:", err);

      const msg = String(err);
      onEvent({ type: "error", message: msg });
      onError?.(msg);
    });
  };

  return {
    loadConfig,
    setChannel,
    checkUpdate,
    updateStream,
  };
}
