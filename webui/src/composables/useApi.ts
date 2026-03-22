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
    if (ksuExec) return ksuExec;
    throw new Error("KernelSU exec not available");
  };

  const ensureSpawn = async (): Promise<KsuSpawn> => {
    await initKernelSU();
    if (ksuSpawn) return ksuSpawn;
    throw new Error("KernelSU spawn not available");
  };

  const execOrThrow = async (cmd: string) => {
    const { errno, stdout, stderr } = await (await ensureExec())(cmd);

    if (errno !== 0) {
      throw new Error(stderr?.trim() || `Command failed: ${cmd}`);
    }

    return stdout.trim();
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
    const stdout = await execOrThrow(
      `find ${PATHS.TARGET_DIR} -mindepth 1 -maxdepth 1 -type d | wc -l`,
    );

    const num = parseInt(stdout, 10);
    return Number.isNaN(num) ? 0 : num;
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

      const handleParsed = (parsed: any) => {
        if (parsed?.type === "info" && parsed?.value === "version") {
          newVersion = parsed.version;
        }
        onEvent(parsed);
      };

      const handleLine = (line: string) => {
        const trimmed = line.trim();
        if (!trimmed) return;

        try {
          handleParsed(JSON.parse(trimmed));
        } catch {
          onEvent({ type: "log", message: trimmed });
        }
      };

      const tryParseJSON = (input: string): [any[], string] => {
        const results: any[] = [];

        while (input) {
          input = input.trimStart();
          if (!input) break;

          try {
            results.push(JSON.parse(input));
            return [results, ""];
          } catch {}

          let splitIndex = -1;

          for (let i = 1; i < input.length; i++) {
            try {
              JSON.parse(input.slice(0, i));
              splitIndex = i;
              break;
            } catch {}
          }

          if (splitIndex === -1) break;

          results.push(JSON.parse(input.slice(0, splitIndex)));
          input = input.slice(splitIndex);
        }

        return [results, input];
      };

      const handleStdout = (chunk: string) => {
        buffer += chunk;

        if (buffer.includes("\n")) {
          const lines = buffer.split("\n");
          buffer = lines.pop() || "";

          lines.forEach(handleLine);
          return;
        }

        const [parsedList, rest] = tryParseJSON(buffer);
        buffer = rest;

        parsedList.forEach(handleParsed);
      };

      const handleStderr = (err: string) => {
        const msg = err.trim();
        if (!msg) return;

        onEvent({ type: "error", message: msg });
        onError?.(msg);
      };

      const handleExit = async (code: number) => {
        if (code !== 0) {
          const msg = `exit code ${code}`;
          onEvent({ type: "error", message: msg });
          onError?.(msg);
          reject(new Error(msg));
          return;
        }

        onEvent({ type: "done" });

        try {
          const version = newVersion ?? (await checkUpdate()).latest_version;

          await setIconsVersion(version);
        } catch {}

        onDone?.();
        resolve();
      };

      const handleError = (err: any) => {
        const msg = String(err);
        onEvent({ type: "error", message: msg });
        onError?.(msg);
        reject(err);
      };

      child.stdout.on("data", handleStdout);
      child.stderr.on("data", handleStderr);
      child.on("exit", handleExit);
      child.on("error", handleError);
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
