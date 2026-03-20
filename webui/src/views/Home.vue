<template>
  <div class="card">
    <h3>运行状态</h3>
    <p>
      Channel: <b>{{ config.default.channel }}</b>
    </p>
    <p>
      Icons Version: <b>{{ config.default.icons_version }}</b>
    </p>
  </div>
</template>

<script setup lang="ts">
defineProps<{ config: any }>();
</script>
