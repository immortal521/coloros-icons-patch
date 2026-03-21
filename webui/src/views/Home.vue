<script setup lang="ts">
import { ref, onMounted } from "vue";

import "@material/web/button/filled-button";
import "@material/web/progress/circular-progress";
import "@material/web/progress/linear-progress";

import { useAPI } from "../composables/useApi";
import { ICONS } from "../constants";

/* ---------------- 类型 ---------------- */

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

/* ---------------- API ---------------- */

const api = useAPI();

/* ---------------- 状态 ---------------- */

const loading = ref(true);
const checking = ref(true); // ✅ 检查更新中
const update = ref<UpdateInfo | null>(null);

const iconCount = ref<number | null>(null); // ✅ 新增

const updating = ref(false);
const progress = ref(0);
const stage = ref("");

/* ---------------- 阶段权重 ---------------- */

const STAGE_WEIGHT: Record<string, { start: number; span: number }> = {
  fetch: { start: 0, span: 5 },
  download: { start: 5, span: 60 },
  verify: { start: 65, span: 5 },
  extract: { start: 70, span: 30 },
};

const STAGE_LABEL: Record<string, string> = {
  fetch: "获取更新信息",
  download: "下载中",
  verify: "校验中",
  extract: "解压中",
};

/* ---------------- 生命周期 ---------------- */

onMounted(async () => {
  try {
    // 并行执行
    const [updateRes, count] = await Promise.all([
      api.checkUpdate().catch(() => null),
      api.getPackagesCount?.().catch(() => null), // ✅ 新 API
    ]);

    if (updateRes) {
      update.value = updateRes;
    }

    if (typeof count === "number") {
      iconCount.value = count;
    }
  } finally {
    loading.value = false;
    checking.value = false;
  }
});

/* ---------------- utils ---------------- */

const formatDate = (iso: string) => new Date(iso).toLocaleString();

const formatSize = (bytes: number) => {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(2)} MB`;
};

/* ---------------- 统一进度 ---------------- */

const updateProgress = (stageName: string, value: number) => {
  const s = STAGE_WEIGHT[stageName];
  if (!s) return;

  const p = Math.min(value, 100);
  const global = Math.floor(s.start + (p / 100) * s.span);

  progress.value = Math.max(progress.value, global);
};

/* ---------------- 更新 ---------------- */

const handleUpdate = async () => {
  updating.value = true;
  progress.value = 0;
  stage.value = "准备中";

  try {
    await api.updateStream((msg) => {
      switch (msg.type) {
        case "stage":
          stage.value = STAGE_LABEL[msg.value as string] ?? msg.value;
          break;

        case "progress":
          updateProgress(msg.stage as string, msg.value as number);
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

    // 刷新
    const [info, count] = await Promise.all([api.checkUpdate(), api.getPackagesCount?.()]);

    update.value = info;
    iconCount.value = count;
  } catch (e) {
    console.error("更新失败:", e);
  } finally {
    updating.value = false;
  }
};
</script>

<template>
  <div class="container">
    <div class="info" style="margin-top: 12px">
      <div class="packages-count-card">
        <span>适配图标</span>
        <span v-if="iconCount !== null">{{ iconCount }}</span>
        <span v-else>计算中...</span>
      </div>
    </div>
    <div class="update-card">
      <div v-if="checking" style="display: flex">
        <div>检查更新中...</div>

        <md-circular-progress v-if="loading" indeterminate style="margin-left: auto" />
      </div>
      <template v-if="update">
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

        <md-filled-button :disabled="updating" @click="handleUpdate" class="update-btn">
          <md-icon slot="icon">
            <svg viewBox="0 0 24 24">
              <path :d="ICONS.update" />
            </svg>
          </md-icon>
          立即更新
        </md-filled-button>
      </template>
      <div v-else>当前已是最新版本</div>
    </div>
  </div>
</template>

<style scoped>
.container {
  width: 100%;
}

/* 卡片 */
.update-card {
  width: 100%;
  border-radius: 16px;
  padding: 20px;

  color: var(--md-sys-color-on-secondary-container);
  background: var(--md-sys-color-secondary-container);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

/* 标题 */
.title {
  margin: 0 0 16px;
  font-size: 20px;
  text-align: center;
}

/* 信息区 */
.info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.packages-count-card {
  display: flex;
  justify-content: space-between;
  font-size: 14px;
  width: 100%;
  margin-bottom: 10px;
  padding: 20px;
  background: var(--md-sys-color-surface-container);
  border-radius: 16px;
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

/* 按钮 */
.update-btn {
  margin-top: 16px;
  width: 100%;
}
</style>
