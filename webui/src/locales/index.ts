export const messages = {
  zh: {
    common: {
      ok: "确定",
      cancel: "取消",
      loading: "加载中...",
      calculating: "计算中...",
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
      packagesCount: "适配图标",
    },

    update: {
      checking: "检查更新中...",
      upToDate: "当前已是最新版本",
      found: "发现更新",

      currentVersion: "当前版本",
      latestVersion: "最新版本",
      publishedAt: "发布时间",
      size: "大小",

      updateNow: "立即更新",

      // stages
      preparing: "准备中",
      fetch: "获取更新信息",
      download: "下载中",
      verify: "校验中",
      extract: "解压中",
      done: "完成",
      failed: "更新失败",
    },

    about: {
      github: "项目地址",
    },

    channel: {
      stable: "稳定版",
      beta: "测试版",
    },
  },

  en: {
    common: {
      ok: "OK",
      cancel: "Cancel",
      loading: "Loading...",
      calculating: "Calculating...",
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
      packagesCount: "Icons",
    },

    update: {
      checking: "Checking for updates...",
      upToDate: "You're up to date",
      found: "Update Available",

      currentVersion: "Current Version",
      latestVersion: "Latest Version",
      publishedAt: "Published At",
      size: "Size",

      updateNow: "Update Now",

      // stages
      preparing: "Preparing",
      fetch: "Fetching info",
      download: "Downloading",
      verify: "Verifying",
      extract: "Extracting",
      done: "Completed",
      failed: "Update Failed",
    },

    about: {
      github: "GitHub",
    },

    channel: {
      stable: "Stable",
      beta: "Beta",
    },
  },
};

export type Locale = keyof typeof messages;
