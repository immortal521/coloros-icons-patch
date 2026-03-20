<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";

const showLangMenu = ref(false);
const langButtonRef = ref<HTMLElement | null>(null);
const menuRef = ref<HTMLElement | null>(null);

// function setLang(code: string) {
//   showLangMenu.value = false;
// }

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
          <!-- <svg viewBox="0 0 24 24"> -->
          <!--   <path :d="ICONS.translate" fill="currentColor" /> -->
          <!-- </svg> -->
        </button>

        <div v-if="showLangMenu" class="menu-dropdown" ref="menuRef">
          <!-- <button -->
          <!--   v-for="l in store.availableLanguages" -->
          <!--   :key="l.code" -->
          <!--   class="menu-item" -->
          <!--   @click="setLang(l.code)" -->
          <!-- > -->
          <!--   {{ l.name }} -->
          <!-- </button> -->
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
  height: 120px;
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
  padding: 24px;
  box-sizing: border-box;
  display: flex;
  align-items: center;
  position: relative;
}
.screen-title {
  flex-grow: 1;
  margin: 0;
  font-size: 32px;
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
