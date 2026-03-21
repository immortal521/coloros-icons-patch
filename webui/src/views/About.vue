<script setup lang="ts">
import { useI18n } from "../composables/useI18n";

const { t } = useI18n();
</script>

<template>
  <div class="container">
    <h4 class="title">{{ t("label.moduleVersion") }}</h4>
    <div>0.1.3</div>
    <h4 class="title">{{ t("label.iconsVersion") }}</h4>
    <div>0.1.3</div>
    <button>项目地址</button>
  </div>
</template>

<style scoped>
.title {
  font-weight: 400;
  line-height: 32px;
}
</style>
