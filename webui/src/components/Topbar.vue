<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import { useI18n } from "../composables/useI18n";
import type { Locale } from "../locales";

const showLangMenu = ref(false);
const langButtonRef = ref<HTMLElement | null>(null);
const menuRef = ref<HTMLElement | null>(null);

const { getLocales, setLocale } = useI18n();

function setLang(code: Locale) {
  setLocale(code);
  showLangMenu.value = false;
}

function handleOutsideClick(e: MouseEvent) {
  const target = e.target as Node;
  if (
    showLangMenu.value &&
    menuRef.value &&
    !menuRef.value.contains(target) &&
    langButtonRef.value &&
    !langButtonRef.value.contains(target)
  ) {
    showLangMenu.value = false;
  }
}

onMounted(() => {
  window.addEventListener("click", handleOutsideClick);
});

onBeforeUnmount(() => {
  window.removeEventListener("click", handleOutsideClick);
});
</script>

<template>
  <header class="top-bar">
    <div class="top-bar-content">
      <h1 class="screen-title">ColorOS Icons Patch</h1>
      <div class="top-actions">
        <button class="btn-icon" ref="langButtonRef" @click="showLangMenu = !showLangMenu">
          <svg viewBox="0 0 24 24">
            <path
              d="M12.65 15.67c.14-.36.05-.77-.23-1.05l-2.09-2.06l.03-.03A17.5 17.5 0 0 0 14.07 6h1.94c.54 0 .99-.45.99-.99v-.02c0-.54-.45-.99-.99-.99H10V3c0-.55-.45-1-1-1s-1 .45-1 1v1H1.99c-.54 0-.99.45-.99.99c0 .55.45.99.99.99h10.18A15.7 15.7 0 0 1 9 11.35c-.81-.89-1.49-1.86-2.06-2.88A.89.89 0 0 0 6.16 8c-.69 0-1.13.75-.79 1.35c.63 1.13 1.4 2.21 2.3 3.21L3.3 16.87a.99.99 0 0 0 0 1.42c.39.39 1.02.39 1.42 0L9 14l2.02 2.02c.51.51 1.38.32 1.63-.35M17.5 10c-.6 0-1.14.37-1.35.94l-3.67 9.8c-.24.61.22 1.26.87 1.26c.39 0 .74-.24.88-.61l.89-2.39h4.75l.9 2.39c.14.36.49.61.88.61c.65 0 1.11-.65.88-1.26l-3.67-9.8c-.22-.57-.76-.94-1.36-.94m-1.62 7l1.62-4.33L19.12 17z"
              fill="currentColor"
            />
          </svg>
        </button>

        <div v-if="showLangMenu" class="menu-dropdown" ref="menuRef">
          <button v-for="l in getLocales()" :key="l" class="menu-item" @click="setLang(l)">
            {{ l }}
          </button>
        </div>
      </div>
    </div>
  </header>
</template>

<style scoped>
.top-bar {
  flex-shrink: 0;
  background-color: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
  height: 100px;
  padding-top: var(--safe-area-inset-top, 0px);
  transition:
    background-color 0.3s ease,
    height 0.3s ease;
  position: relative;
  z-index: 20;
  display: flex;
  align-items: center;
  overflow: visible;
}

.top-bar-content {
  height: 100%;
  width: 100%;
  padding: 18px;
  box-sizing: border-box;
  display: flex;
  align-items: center;
  position: relative;
}
.screen-title {
  flex-grow: 1;
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  letter-spacing: 0;
  transform-origin: left bottom;
  transition: all 0.3s cubic-bezier(0.2, 0, 0, 1);
  white-space: nowrap;
}
.top-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  z-index: 50;
}
.btn-icon {
  width: 48px;
  height: 48px;
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 0;
  border-radius: 50%;
  color: var(--md-sys-color-on-surface-variant);
  display: flex;
  align-items: center;
  justify-content: center;
}
.btn-icon:hover {
  background-color: var(--md-sys-color-surface-variant);
}
.btn-icon svg {
  width: 24px;
  height: 24px;
}
.menu-dropdown {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 4px;
  background: var(--md-sys-color-surface-container-high);
  border-radius: 16px;
  padding: 8px 0;
  z-index: 1000;
  min-width: 200px;
  max-height: 300px;
  overflow-y: auto;
  overflow-x: hidden;
}
.menu-item {
  width: 100%;
  padding: 16px 24px;
  text-align: left;
  background: none;
  border: none;
  color: var(--md-sys-color-on-surface);
  cursor: pointer;
  font-weight: 500;
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}
.menu-item:hover {
  background: var(--md-sys-color-surface-variant);
}
</style>
