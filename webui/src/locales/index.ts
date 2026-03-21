export const messages = {
  zh: {
    common: {
      ok: "确定",
      cancel: "取消",
    },
    tab: {
      home: "状态",
      settings: "配置",
      about: "关于",
    },
    label: {
      channel: "更新通道",
      channelDesc: "设置图标更新的通道",
      moduleVersion: "模块版本",
      iconsVersion: "图标版本",
    },
  },
  en: {
    common: {
      ok: "OK",
      cancel: "Cancel",
    },
    tab: {
      home: "Status",
      settings: "Settings",
      about: "About",
    },
    label: {
      channel: "Update Channel",
      channelDesc: "Set the update channel for icons",
      moduleVersion: "Module Version",
      iconsVersion: "Icons Version",
    },
  },
};

export type Locale = keyof typeof messages;
