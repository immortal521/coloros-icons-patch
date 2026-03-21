import { ref } from "vue";
import type { Channel, Config } from "../types/config";

const config = ref<Config>({
  default: {
    icons_version: "",
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
});
const setChannel = (channel: Channel) => {
  config.value.default.channel = channel;
};

export const useConfigStore = () => {
  return {
    config,
    setChannel,
  };
};
