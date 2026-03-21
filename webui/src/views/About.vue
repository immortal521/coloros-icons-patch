<script setup lang="ts">
import { useI18n } from "../composables/useI18n";
import { ICONS, PATHS } from "../constants";
import { useConfigStore } from "../stores/configStore";
import "@material/web/button/filled-button";

const { t } = useI18n();

const { config } = useConfigStore();

const toGithub = () => {};
</script>

<template>
  <div class="container">
    <h4 class="title">{{ t("label.moduleVersion") }}</h4>
    <div>{{ PATHS.VERSION }}</div>
    <h4 class="title">{{ t("label.iconsVersion") }}</h4>
    <div>{{ config.default.icons_version }}</div>
    <md-filled-button @click="toGithub" class="github-btn">
      <md-icon slot="icon">
        <svg viewBox="0 0 24 24">
          <path :d="ICONS.github" />
        </svg>
      </md-icon>
      {{ t("about.github") }}
    </md-filled-button>
  </div>
</template>

<style scoped>
.title {
  font-weight: 400;
  line-height: 32px;
}

.github-btn {
  width: 100%;
}
</style>
