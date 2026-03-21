<template>
  <div class="container">
    <!-- loading -->
    <md-circular-progress v-if="loading" indeterminate />

    <!-- update card -->
    <Card v-if="!loading && update" class="update-card">
      <h2 class="title">发现更新</h2>

      <div class="info">
        <div class="row">
          <span>当前版本</span>
          <span>{{ update.current_version }}</span>
        </div>
        <div class="row">
          <span>最新版本</span>
          <span class="highlight">{{ update.latest_version }}</span>
        </div>
        <div class="row">
          <span>发布时间</span>
          <span>{{ formatDate(update.published_at) }}</span>
        </div>
        <div class="row">
          <span>大小</span>
          <span>{{ formatSize(update.update_size) }}</span>
        </div>
      </div>

      <!-- 进度 -->
      <div v-if="updating" class="progress">
        <div class="stage">{{ stage }}</div>

        <md-linear-progress :value="progress / 100"></md-linear-progress>

        <div class="percent">{{ progress }}%</div>
      </div>

      <div v-if="update.notes" class="notes">
        {{ update.notes }}
      </div>

      <div class="actions">
        <md-filled-button :disabled="updating" @click="handleUpdate">
          {{ updating ? "更新中..." : "立即更新" }}
        </md-filled-button>
      </div>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";

import "@material/web/button/filled-button";
import "@material/web/progress/circular-progress";
import "@material/web/progress/linear-progress";

import { useAPI } from "../composables/useApi";
import Card from "../components/Card.vue";

interface UpdateInfo {
  current_version: string;
  latest_version: string;
  update_url: string;
  checksum: string;
  has_update: boolean;
  update_name: string;
  update_size: number;
  published_at: string;
  notes: string;
  revision: number;
}

const api = useAPI();

const loading = ref(true);
const update = ref<UpdateInfo | null>(null);

/* ---------- 新增状态 ---------- */
const updating = ref(false);
const progress = ref(0);
const stage = ref("");

/* ---------- 生命周期 ---------- */

onMounted(async () => {
  try {
    const res = await api.checkUpdate();
    if (res.has_update) update.value = res;
  } catch (e) {
    console.error("check update failed:", e);
  } finally {
    loading.value = false;
  }
});

/* ---------- utils ---------- */

const formatDate = (iso: string) => new Date(iso).toLocaleString();

const formatSize = (bytes: number) => {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(2)} MB`;
};

/* ---------- 核心：流式更新 ---------- */

const handleUpdate = async () => {
  updating.value = true;
  progress.value = 0;
  stage.value = "准备中";

  try {
    await api.updateStream((msg) => {
      console.log("update msg:", msg);

      switch (msg.type) {
        case "stage":
          stage.value = msg.value as string;
          break;

        case "progress":
          progress.value = msg.value as number;
          break;

        case "info":
          if (msg.message) stage.value = msg.message;
          break;

        case "done":
          stage.value = "完成";
          progress.value = 100;
          break;
      }
    });

    // 刷新状态
    const info = await api.checkUpdate();
    update.value = info;
  } catch (e) {
    console.error("更新失败:", e);
  } finally {
    updating.value = false;
  }
};
</script>

<style scoped>
.container {
  padding: 16px;
  width: 100%;
}

/* 卡片 */
.update-card {
  width: 100%;
  max-width: 100%;
  border-radius: 16px;
  padding: 20px;

  background: var(--md-sys-color-surface);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

/* 标题 */
.title {
  margin: 0 0 16px;
  font-size: 20px;
}

/* 信息区 */
.info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.row {
  display: flex;
  justify-content: space-between;
  font-size: 14px;
  opacity: 0.9;
}

.highlight {
  color: var(--md-sys-color-primary);
  font-weight: 500;
}

/* 进度 */
.progress {
  margin-top: 16px;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.stage {
  font-size: 13px;
  opacity: 0.7;
}

.percent {
  font-size: 12px;
  text-align: right;
  opacity: 0.6;
}

/* 更新说明 */
.notes {
  margin-top: 12px;
  font-size: 13px;
  opacity: 0.75;
}

/* 按钮区 */
.actions {
  margin-top: 20px;
  display: flex;
  justify-content: flex-end;
}
</style>
