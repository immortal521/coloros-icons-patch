<script setup lang="ts">
import { ref, watch, nextTick } from "vue";
import { useI18n } from "../composables/useI18n";
import { tabs, type TabKey } from "../tabs";

interface Props {
  activeTab: TabKey;
  onTabChange: (id: TabKey) => void;
}

const props = defineProps<Props>();
const navContainer = ref<HTMLElement | null>(null);
const tabRefs = ref<Record<string, HTMLElement>>({});

const { t } = useI18n();

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

function setTab(id: TabKey) {
  props.onTabChange(id);
}
</script>

<template>
  <nav :class="['bottom-nav']" ref="navContainer">
    <button
      v-for="tab in tabs"
      :key="tab.key"
      :class="['nav-tab', props.activeTab === tab.key ? 'active' : '']"
      @click="setTab(tab.key)"
      type="button"
      ref="el => (tabRefs[tab.id] = el)"
    >
      <div class="icon-container">
        <svg viewBox="0 0 24 24">
          <path :d="tab.icon" />
        </svg>
      </div>
      <span class="label">{{ t(tab.label) }}</span>
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
