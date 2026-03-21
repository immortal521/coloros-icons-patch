<template>
  <div class="card md-ripple" :style="style">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

import "@material/web/elevation/elevation";
import "@material/web/ripple/ripple";

const props = defineProps<{
  level?: number;
}>();

const style = computed(() => ({
  "--md-elevation-level": props.level ?? 2,
}));
</script>

<style scoped>
.card {
  border-radius: 16px;
  padding: 20px;

  background: var(--md-sys-color-surface);
  color: var(--md-sys-color-on-surface);

  box-shadow: var(--md-elevation-level);

  transition:
    box-shadow 0.2s,
    transform 0.15s;
  cursor: pointer;
}

.card:hover {
  --md-elevation-level: 3;
  transform: translateY(-2px);
}

.card:active {
  --md-elevation-level: 1;
  transform: scale(0.98);
}
</style>
