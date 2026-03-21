export interface Config {
  default: {
    icons_version: string;
    channel: "stable" | "beta";
    runtime_dir: string;
    temp_dir: string;
    target_dir: string;
  };
  source: {
    beta: Source;
    stable: Source;
  };
}

export type Channel = "stable" | "beta";

export interface Source {
  url: string;
}
