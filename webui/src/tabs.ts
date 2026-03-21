import Home from "./views/Home.vue";
import Settings from "./views/Settings.vue";
import About from "./views/About.vue";

export type TabKey = "home" | "settings" | "about";

export type TabItem = {
  key: TabKey;
  label: string;
  icon?: string;
  component: any;
};

export const tabs: TabItem[] = [
  {
    key: "home",
    label: "tab.home",
    component: Home,
  },
  {
    key: "settings",
    label: "tab.settings",
    component: Settings,
  },
  {
    key: "about",
    label: "tab.about",
    component: About,
  },
];
