<template>
  <div class="layout">
    <Topbar />
    <main class="content">
      <Home v-if="page === 'home'" :config="config" />
      <Settings v-else :config="config" @update="updateConfig" />
    </main>
    <Navbar :activeTab="page" @tabChange="setPage" />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import Home from "./views/Home.vue";
import Settings from "./views/Settings.vue";
import Topbar from "./components/Topbar.vue";
import Navbar from "./components/Navbar.vue";

const page = ref<"home" | "settings">("home");

const setPage = (p: "home" | "settings") => {
  page.value = p;
};

const config = ref({
  default: {
    icons_version: "0.1.0",
    channel: "beta",
    runtime_dir: "./",
    temp_dir: "./runtime",
    target_dir: "./icons",
  },
});

function updateConfig(newCfg: any) {
  config.value.default = newCfg;
}
</script>

<style scoped>
.app-root {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  background-color: var(--md-sys-color-background);
}
.layout {
  height: 100vh;
}

.content {
  flex: 1;
  overflow-x: hidden;
  position: relative;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  height: calc(100% - 200px);
}
.swipe-track {
  display: flex;
  width: 500%;
  height: 100%;
  will-change: transform;
}
.swipe-page {
  width: 20%;
  height: 100%;
  flex-shrink: 0;
  position: relative;
  overflow: hidden;
}
.page-scroller {
  height: 100%;
  overflow-y: auto;
  padding: 16px;
  padding-bottom: calc(16px + var(--safe-area-inset-bottom, 0px));
  box-sizing: border-box;
  width: 100%;
  max-width: 800px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
}
* {
  scrollbar-width: none;
  -ms-overflow-style: none;
}
::-webkit-scrollbar {
  display: none;
}
</style>
