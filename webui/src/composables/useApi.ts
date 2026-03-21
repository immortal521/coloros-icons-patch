import { PATHS } from "../constants";
import type { Config } from "../types/config";

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

export type UpdateEvent =
  | { type: "stage"; value: string }
  | { type: "progress"; stage: string; value: number }
  | { type: "info"; value?: string; version?: string; message?: string }
  | { type: "done" }
  | { type: "log"; message: string }
  | { type: "error"; message: string };

let ksuExec: KsuExec | null = null;
let ksuSpawn: KsuSpawn | null = null;
let initPromise: Promise<void> | null = null;

const initKernelSU = async () => {
  if (ksuExec && ksuSpawn) return;

  if (!initPromise) {
    initPromise = (async () => {
      if (import.meta.env.DEV) {
        console.warn("[KernelSU] DEV MODE → using MOCK");

        const mock = await import("../mocks/kernelsuMock");

        ksuExec = mock.mockExec;
        ksuSpawn = mock.mockSpawn;
        return;
      }

      try {
        const mod = (await import("kernelsu")) as any;
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

export function useAPI() {
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

  const execOrThrow = async (cmd: string) => {
    const exec = await ensureExec();
    const res = await exec(cmd);

    if (res.errno !== 0) {
      throw new Error(res.stderr?.trim() || `Command failed: ${cmd}`);
    }

    return res.stdout.trim();
  };

  const loadConfig = async (): Promise<Config> => {
    const stdout = await execOrThrow(`${PATHS.CIP_BIN} config get --config ${PATHS.CONFIG} --json`);
    return JSON.parse(stdout);
  };

  const setChannel = async (channel: string) => {
    await execOrThrow(`${PATHS.CIP_BIN} config set --config ${PATHS.CONFIG} --channel ${channel}`);
  };

  const setIconsVersion = async (version: string) => {
    await execOrThrow(
      `${PATHS.CIP_BIN} config set --config ${PATHS.CONFIG} --icons-version ${version}`,
    );
  };

  const getPackagesCount = async (): Promise<number> => {
    const cmd = `find ${PATHS.TARGET_DIR} -mindepth 1 -maxdepth 1 -type d | wc -l`;

    const stdout = await execOrThrow(cmd);

    const num = parseInt(stdout.trim(), 10);
    if (isNaN(num)) return 0;

    return num;
  };

  const checkUpdate = async (): Promise<UpdateInfo> => {
    const stdout = await execOrThrow(`${PATHS.CIP_BIN} check --config ${PATHS.CONFIG} --json`);
    return JSON.parse(stdout);
  };

  const updateStream = async (
    onEvent: (e: UpdateEvent) => void,
    onDone?: () => void,
    onError?: (err: string) => void,
  ): Promise<void> => {
    const spawn = await ensureSpawn();

    return new Promise((resolve, reject) => {
      const child = spawn(PATHS.CIP_BIN, ["update", "--config", PATHS.CONFIG, "--json"]);

      let newVersion: string | null = null;
      let buffer = "";

      child.stdout.on("data", (chunk: string) => {
        console.log("[raw stdout]", chunk);
        buffer += chunk;

        const lines = buffer.split("\n");
        buffer = lines.pop() || "";

        for (const line of lines) {
          const trimmed = line.trim();
          if (!trimmed) continue;

          try {
            const parsed = JSON.parse(trimmed);

            console.log("[parsed event]", parsed);

            if (parsed.type === "info" && parsed.value === "version") {
              newVersion = parsed.version;
            }

            onEvent(parsed);
          } catch {
            onEvent({ type: "log", message: trimmed });
          }
        }
      });

      child.stderr.on("data", (err: string) => {
        onEvent({ type: "error", message: err.trim() });
        onError?.(err);
      });

      child.on("exit", async (code: number) => {
        if (code === 0) {
          onEvent({ type: "done" });

          try {
            if (newVersion) {
              await setIconsVersion(newVersion);
            } else {
              const info = await checkUpdate();
              await setIconsVersion(info.latest_version);
            }
          } catch {}

          onDone?.();
          resolve();
          return;
        }

        const msg = `exit code ${code}`;
        onEvent({ type: "error", message: msg });
        onError?.(msg);
        reject(new Error(msg));
      });

      child.on("error", (err: any) => {
        const msg = String(err);
        onEvent({ type: "error", message: msg });
        onError?.(msg);
        reject(err);
      });
    });
  };

  return {
    loadConfig,
    setChannel,
    checkUpdate,
    getPackagesCount,
    updateStream,
  };
}
