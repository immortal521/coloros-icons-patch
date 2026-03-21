<script setup lang="ts">
import { useI18n } from "../composables/useI18n";
import { useConfigStore } from "../stores/configStore";
import type { Channel } from "../types/config";

import "@material/web/select/outlined-select.js";
import "@material/web/select/select-option.js";
import "@material/web/icon/icon.js";
import { ICONS } from "../constants";

const { t } = useI18n();
const { config, setChannel } = useConfigStore();

const channelOptions: Channel[] = ["stable", "beta"];

const handleChannelChange = (e: Event) => {
  const value = (e.target as HTMLSelectElement).value as Channel;
  setChannel(value);
};
</script>

<template>
  <div class="config-container">
    <section class="config-card">
      <div class="card-header">
        <div class="card-icon">
          <md-icon>
            <svg viewBox="0 0 24 24">
              <path :d="ICONS.channel" />
            </svg>
          </md-icon>
        </div>

        <div class="card-text">
          <span class="card-title">{{ t("label.channel") }}</span>
          <span class="card-desc">{{ t("label.channelDesc") }}</span>
        </div>
      </div>

      <md-outlined-select
        class="full-width-field"
        :label="t('label.channel')"
        :value="config.default.channel"
        @change="handleChannelChange"
      >
        <md-select-option v-for="item in channelOptions" :key="item" :value="item">
          {{ t(`channel.${item}`) }}
        </md-select-option>

        <md-icon slot="leading-icon">
          <svg viewBox="0 0 24 24">
            <path :d="ICONS.channel" />
          </svg>
        </md-icon>
      </md-outlined-select>
    </section>
  </div>
</template>

<style scoped>
.config-container {
  display: flex;
  flex-direction: column;
  gap: 24px;
  padding-bottom: 24px;
}

.config-card {
  background-color: var(--md-sys-color-surface-container);
  border-radius: 20px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.card-icon {
  width: 40px;
  height: 40px;
  border-radius: 12px;
  background-color: var(--md-sys-color-secondary-container);
  color: var(--md-sys-color-on-secondary-container);
  display: flex;
  align-items: center;
  justify-content: center;
}

.card-text {
  display: flex;
  flex-direction: column;
}

.card-title {
  font-size: 16px;
  font-weight: 500;
  color: var(--md-sys-color-on-surface);
}

.card-desc {
  font-size: 12px;
  color: var(--md-sys-color-on-surface-variant);
}

.full-width-field {
  width: 100%;
  --md-outlined-text-field-container-shape: 16px;
}
</style>
