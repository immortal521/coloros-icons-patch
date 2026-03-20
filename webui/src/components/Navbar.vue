<script setup lang="ts">
import { ref, watch, nextTick } from "vue";

interface Tab {
  id: "home" | "settings";
  icon: string;
  label: string;
}

interface Props {
  activeTab: "home" | "settings";
  onTabChange: (id: "home" | "settings") => void;
}

const props = defineProps<Props>();
const navContainer = ref<HTMLElement | null>(null);
const tabRefs = ref<Record<string, HTMLElement>>({});

// 写死首页和设置 tab
const TABS: Tab[] = [
  { id: "home", icon: "M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z", label: "首页" },
  {
    id: "settings",
    icon: "M19.43 12.98c.04-.32.07-.66.07-1s-.03-.68-.07-1l2.11-1.65a.5.5 0 0 0 .12-.63l-2-3.46a.5.5 0 0 0-.6-.22l-2.49 1a7.07 7.07 0 0 0-1.7-.99l-.38-2.65A.5.5 0 0 0 14 2h-4a.5.5 0 0 0-.5.42l-.38 2.65c-.63.23-1.22.55-1.7.99l-2.49-1a.5.5 0 0 0-.6.22l-2 3.46a.5.5 0 0 0 .12.63L4.57 11c-.04.32-.07.66-.07 1s.03.68.07 1l-2.11 1.65a.5.5 0 0 0-.12.63l2 3.46c.14.24.42.34.68.22l2.49-1c.48.44 1.07.76 1.7.99l.38 2.65c.04.28.28.48.56.48h4c.28 0 .52-.2.56-.48l.38-2.65c.63-.23 1.22-.55 1.7-.99l2.49 1c.26.12.54.02.68-.22l2-3.46a.5.5 0 0 0-.12-.63l-2.11-1.65zM12 15.5a3.5 3.5 0 1 1 0-7 3.5 3.5 0 0 1 0 7z",
    label: "设置",
  },
];

// 监听 activeTab 改变，滚动到中间
watch(
  () => props.activeTab,
  async (activeTab) => {
    await nextTick();
    const container = navContainer.value;
    const tab = tabRefs.value[activeTab];
    if (container && tab) {
      const containerWidth = container.clientWidth;
      const tabLeft = tab.offsetLeft;
      const tabWidth = tab.clientWidth;
      const scrollLeft = tabLeft - containerWidth / 2 + tabWidth / 2;
      container.scrollTo({ left: scrollLeft, behavior: "smooth" });
    }
  },
  { immediate: true },
);

function setTab(id: "home" | "settings") {
  props.onTabChange(id);
}
</script>

<template>
  <nav :class="['bottom-nav']" ref="navContainer">
    <button
      v-for="tab in TABS"
      :key="tab.id"
      :class="['nav-tab', props.activeTab === tab.id ? 'active' : '']"
      @click="setTab(tab.id)"
      type="button"
      ref="el => (tabRefs[tab.id] = el)"
    >
      <div class="icon-container">
        <svg viewBox="0 0 24 24">
          <path :d="tab.icon" />
        </svg>
      </div>
      <span class="label">{{ tab.label }}</span>
    </button>
  </nav>
</template>

<style scoped>
.bottom-nav {
  display: flex;
  position: fixed;
  bottom: 0;
  width: 100%;
  height: 80px;
  background-color: var(--md-sys-color-surface-container);
  border-top: none;
  padding-bottom: max(16px, var(--safe-area-inset-bottom, 0px));
  padding-top: 16px;
  padding-left: 24px;
  padding-right: 24px;
  overflow-x: auto;
  flex-shrink: 0;
  -ms-overflow-style: none;
  gap: 8px;
  z-index: 100;
  transition: padding-bottom 0.3s ease;
}
.bottom-nav.fix-padding {
  padding-bottom: calc(max(16px, var(--safe-area-inset-bottom, 0px)) + 48px);
}
.bottom-nav::-webkit-scrollbar {
  display: none;
}
.nav-tab {
  flex: 1;
  min-width: 64px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 4px;
  font-size: 12px;
  font-weight: 500;
  color: var(--md-sys-color-on-surface-variant);
  background: transparent;
  cursor: pointer;
  border: none;
  padding: 0;
}
.icon-container {
  width: 64px;
  height: 32px;
  border-radius: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s cubic-bezier(0.2, 0, 0, 1);
}
.nav-tab svg {
  width: 24px;
  height: 24px;
  fill: currentColor;
  transition: all 0.2s;
}
.nav-tab.active .icon-container {
  background-color: var(--md-sys-color-secondary-container);
  width: 64px;
}
.nav-tab.active {
  color: var(--md-sys-color-on-surface);
}
.nav-tab.active svg {
  fill: var(--md-sys-color-on-secondary-container);
}
.nav-tab.active .label {
  font-weight: 700;
}
.nav-tab:active .icon-container {
  opacity: 0.8;
}
</style>
