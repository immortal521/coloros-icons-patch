import type { Config } from "../types/config";

/* ---------------- utils ---------------- */

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

/* ---------------- mock data ---------------- */

const mockConfig: Config = {
  default: {
    icons_version: "1.0.0",
    channel: "stable",
    runtime_dir: "",
    temp_dir: "",
    target_dir: "",
  },
  source: {
    beta: {
      url: "",
    },
    stable: {
      url: "",
    },
  },
};

const mockUpdate = {
  current_version: "1.0.0",
  latest_version: "1.2.0",
  has_update: true,
  update_name: "icons_v1.2.0.zip",
  update_size: 12 * 1024 * 1024,
  published_at: new Date().toISOString(),
  notes: "Mock 更新",
  revision: 1,
};

/* ---------------- exec ---------------- */

export const mockExec = async (cmd: string) => {
  await sleep(200);

  if (cmd.includes("config get")) {
    return { errno: 0, stdout: JSON.stringify(mockConfig), stderr: "" };
  }

  if (cmd.includes("check")) {
    return { errno: 0, stdout: JSON.stringify(mockUpdate), stderr: "" };
  }

  return { errno: 0, stdout: "", stderr: "" };
};

/* ---------------- spawn ---------------- */

export const mockSpawn = () => {
  let stdoutCb: any;
  let exitCb: any;

  (async () => {
    const emit = (o: any) => stdoutCb?.(JSON.stringify(o) + "\n");

    emit({ type: "stage", value: "fetch" });
    await sleep(300);

    emit({ type: "info", value: "version", version: "1.2.0" });

    emit({ type: "stage", value: "download" });

    for (let i = 0; i <= 100; i += 10) {
      await sleep(120);
      emit({ type: "progress", stage: "download", value: i });
    }

    emit({ type: "stage", value: "extract" });

    for (let i = 0; i <= 100; i += 20) {
      await sleep(120);
      emit({ type: "progress", stage: "extract", value: i });
    }

    emit({ type: "done" });
    exitCb?.(0);
  })();

  return {
    stdout: {
      on(_: string, cb: any) {
        stdoutCb = cb;
      },
    },
    stderr: {
      on() {},
    },
    on(event: string, cb: any) {
      if (event === "exit") exitCb = cb;
    },
  };
};
