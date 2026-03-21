import type { Config } from "../types/config";

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

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

export const mockSpawn = () => {
  let stdoutCb: any;
  let stderrCb: any;
  let exitCb: any;

  (async () => {
    const emit = (o: any) => {
      stdoutCb?.(JSON.stringify(o) + "\n");
    };

    /* ---------- fetch ---------- */
    emit({
      type: "stage",
      value: "fetch",
      message:
        "Fetching index: https://immortal521.github.io/coloros-icons-patch/stable/index.json",
    });

    await sleep(400);

    emit({
      type: "info",
      value: "version",
      version: "2026.3.21",
      message: "Latest version: 2026.3.21",
    });

    /* ---------- download ---------- */
    emit({ type: "stage", value: "download" });

    for (let i = 0; i <= 100; i += 2) {
      await sleep(60);
      emit({
        type: "progress",
        stage: "download",
        value: i,
      });
    }

    /* ---------- verify ---------- */
    emit({
      type: "stage",
      value: "verify",
      message: "Verifying SHA256",
    });

    await sleep(500);

    emit({
      type: "info",
      value: "verify_ok",
      message: "SHA256 OK",
    });

    /* ---------- extract ---------- */
    emit({ type: "stage", value: "extract" });

    const total = 30; // 模拟30个文件

    for (let i = 0; i <= total; i++) {
      await sleep(50);

      emit({
        type: "progress",
        stage: "extract",
        value: Math.floor((i / total) * 100),
        file: `com.example.app${i}/icon.png`,
      });
    }

    /* ---------- done ---------- */
    emit({
      type: "done",
      target: "/data/adb/modules/ColorOSIconsPatch/uxicons",
    });

    await sleep(200);

    exitCb?.(0);
  })();

  return {
    stdout: {
      on(event: "data", cb: any) {
        if (event === "data") stdoutCb = cb;
      },
    },
    stderr: {
      on(event: "data", cb: any) {
        if (event === "data") stderrCb = cb;
      },
    },
    on(event: string, cb: any) {
      if (event === "exit") exitCb = cb;
    },
  };
};
