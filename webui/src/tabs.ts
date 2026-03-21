import Home from "./views/Home.vue";
import Settings from "./views/Settings.vue";
import About from "./views/About.vue";
import { ICONS } from "./constants";

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
    icon: ICONS.home,
  },
  {
    key: "settings",
    label: "tab.settings",
    component: Settings,
    icon: ICONS.settings,
  },
  {
    key: "about",
    label: "tab.about",
    component: About,
    icon: ICONS.about,
  },
];
