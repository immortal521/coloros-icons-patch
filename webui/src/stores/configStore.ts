import { ref, readonly } from "vue";
import type { Channel, Config } from "../types/config";
import { useAPI } from "../composables/useApi";

const defaultConfig: Config = {
  default: {
    icons_version: "",
    channel: "stable",
    runtime_dir: "",
    temp_dir: "",
    target_dir: "",
  },
  source: {
    beta: { url: "" },
    stable: { url: "" },
  },
};

const config = ref<Config>({ ...defaultConfig });
const loading = ref(false);
const error = ref<string | null>(null);

let fetchPromise: Promise<Config> | null = null;

const api = useAPI();

const mergeConfig = (remote: Partial<Config>): Config => {
  return {
    ...defaultConfig,
    ...remote,
    default: {
      ...defaultConfig.default,
      ...remote.default,
    },
    source: {
      ...defaultConfig.source,
      ...remote.source,
      beta: {
        ...defaultConfig.source.beta,
        ...remote.source?.beta,
      },
      stable: {
        ...defaultConfig.source.stable,
        ...remote.source?.stable,
      },
    },
  };
};

const getConfig = async (): Promise<Config> => {
  if (fetchPromise) return fetchPromise;

  loading.value = true;
  error.value = null;

  fetchPromise = (async () => {
    try {
      const remote = await api.loadConfig();
      const merged = mergeConfig(remote);
      config.value = merged;
      return merged;
    } catch (e: any) {
      error.value = e.message || "Failed to load config";
      throw e;
    } finally {
      loading.value = false;
      fetchPromise = null;
    }
  })();

  return fetchPromise;
};

const setChannel = async (channel: Channel) => {
  const prev = config.value.default.channel;

  config.value.default.channel = channel;

  try {
    await api.setChannel(channel);
  } catch (e) {
    config.value.default.channel = prev;
    throw e;
  }
};

export const useConfigStore = () => {
  return {
    config: readonly(config),
    loading: readonly(loading),
    error: readonly(error),
    getConfig,
    setChannel,
  };
};
