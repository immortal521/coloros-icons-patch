<script setup lang="ts">
import { useI18n } from "../composables/useI18n";
import { useConfigStore } from "../stores/configStore";

const { t } = useI18n();

const { config } = useConfigStore();
</script>

<template>
  <div class="container">
    <h4 class="title">{{ t("label.moduleVersion") }}</h4>
    <div>0.1.3</div>
    <h4 class="title">{{ t("label.iconsVersion") }}</h4>
    <div>{{ config.default.icons_version }}</div>
    <button>项目地址</button>
  </div>
</template>

<style scoped>
.title {
  font-weight: 400;
  line-height: 32px;
}
</style>
