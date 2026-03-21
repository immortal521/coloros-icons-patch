export const messages = {
  zh: {
    common: {
      ok: "确定",
      cancel: "取消",
    },
    tab: {
      home: "首页",
      settings: "配置",
      about: "关于",
    },
  },
  en: {
    common: {
      ok: "OK",
      cancel: "Cancel",
    },
    tab: {
      home: "Home",
      settings: "Settings",
      about: "About",
    },
  },
};

export type Locale = keyof typeof messages;
