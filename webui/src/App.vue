<script setup lang="ts">
import { ref, onMounted } from "vue";
import Topbar from "./components/Topbar.vue";
import Navbar from "./components/Navbar.vue";
import { tabs, type TabKey } from "./tabs";

const page = ref<TabKey>("home");
const contentRef = ref<HTMLElement | null>(null);

function setPage(p: TabKey) {
  page.value = p;

  const index = tabs.findIndex((t) => t.key === p);
  const el = contentRef.value;

  if (el) {
    el.scrollTo({
      left: index * el.clientWidth,
      behavior: "smooth",
    });
  }
}

let scrollTimer: number | null = null;
function onScroll() {
  if (scrollTimer) {
    clearTimeout(scrollTimer);
  }

  scrollTimer = window.setTimeout(() => {
    const el = contentRef.value;
    if (!el) return;

    const index = Math.round(el.scrollLeft / el.clientWidth);
    const tab = tabs[index];

    if (tab && tab.key !== page.value) {
      page.value = tab.key;
    }
  }, 100);
}

onMounted(() => {
  contentRef.value?.addEventListener("scroll", onScroll);
});
</script>

<template>
  <div class="layout">
    <Topbar :activeTab="page" />

    <main class="content" ref="contentRef">
      <div v-for="tab in tabs" :key="tab.key" class="page">
        <component :is="tab.component" />
      </div>
    </main>

    <Navbar :activeTab="page" @tabChange="setPage" />
  </div>
</template>

<style scoped>
.layout {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.content {
  flex: 1;
  display: flex;

  overflow-x: auto;
  overflow-y: hidden;

  scroll-snap-type: x mandatory;
  scroll-behavior: smooth;
}

.page {
  width: 100vw;
  flex-shrink: 0;

  scroll-snap-align: start;
}
</style>
