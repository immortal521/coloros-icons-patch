<script setup lang="ts">
import { ref, computed, useTemplateRef, onMounted } from "vue";
import Topbar from "./components/Topbar.vue";
import Navbar from "./components/Navbar.vue";
import { tabs, type TabKey } from "./tabs";

const page = ref<TabKey>("home");

const currentIndex = computed(() => tabs.findIndex((t) => t.key === page.value));

function setPage(p: TabKey) {
  page.value = p;
}

let startX = 0;
let startTime = 0;

const deltaX = ref(0);
const isDragging = ref(false);

const contentRef = useTemplateRef<HTMLDivElement>("content");
const width = ref(0);
const pageCount = tabs.length;

const innerStyle = computed(() => {
  const offset = -currentIndex.value * width.value + deltaX.value;

  return {
    transform: `translate3d(${offset}px, 0, 0)`,
    transition: isDragging.value ? "none" : "transform 0.25s ease-out",
  };
});

function onTouchStart(e: TouchEvent) {
  startX = e.touches[0].clientX;
  startTime = Date.now();
  isDragging.value = true;
}

function onTouchMove(e: TouchEvent) {
  const x = e.touches[0].clientX;
  let dx = x - startX;

  if ((currentIndex.value === 0 && dx > 0) || (currentIndex.value === pageCount - 1 && dx < 0)) {
    dx *= 0.35;
  }

  deltaX.value = dx;
}

function onTouchEnd() {
  const duration = Date.now() - startTime;
  const velocity = deltaX.value / duration;

  const threshold = width.value * 0.25;

  let nextIndex = currentIndex.value;

  if (deltaX.value > threshold || velocity > 0.5) {
    nextIndex--;
  } else if (deltaX.value < -threshold || velocity < -0.5) {
    nextIndex++;
  }

  nextIndex = Math.max(0, Math.min(pageCount - 1, nextIndex));

  page.value = tabs[nextIndex].key;

  deltaX.value = 0;
  isDragging.value = false;
}

onMounted(() => {
  if (!contentRef.value) return;

  const observer = new ResizeObserver((entries) => {
    width.value = entries[0].contentRect.width;
  });

  observer.observe(contentRef.value);
});
</script>

<template>
  <div class="layout">
    <Topbar :activeTab="page" />

    <div
      class="content"
      ref="content"
      @touchstart="onTouchStart"
      @touchmove="onTouchMove"
      @touchend="onTouchEnd"
    >
      <div class="pages-inner" :style="innerStyle">
        <div v-for="tab in tabs" :key="tab.key" class="page">
          <component :is="tab.component" />
        </div>
      </div>
    </div>

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
  overflow: hidden;
  position: relative;
}

.pages-inner {
  display: flex;
  height: 100%;
}

.page {
  width: 100vw;
  flex-shrink: 0;
  padding: 10px;
}
</style>
