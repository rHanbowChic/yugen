<script setup lang="ts">
// 这个组件不能再直接添加新内容了（过于臃肿），请把新内容放在其他组件，在这里调用
import { computed, onMounted, reactive, ref, watch } from "vue";
import * as autostart from "@tauri-apps/plugin-autostart";
import { ConfObject } from "../models/ConfObject";
import { getDefaultMinMaxWait, getMusicFromCollection, requireMusicDownload } from "../utils/utils";

const STORAGE_KEY = "yugen_conf";
const SEC = 1000;

type CollectionOption = {
  value: string;
  label: string;
};

const collectionOptions: CollectionOption[] = [
  { value: "ultimate", label: "Ultimate（全曲库）" },
  { value: "overworld", label: "Overworld（主世界）" },
  { value: "overworld_classic", label: "Overworld Classic（经典主世界）" },
  { value: "overworld_creative", label: "Overworld + Creative" },
  { value: "creative", label: "Creative（创造）" },
  { value: "deep_dark", label: "Deep Dark（深暗之域）" },
  { value: "underwater", label: "Underwater（水下）" },
  { value: "nether", label: "Nether（下界）" },
  { value: "nether_classic", label: "Nether Classic（经典下界）" },
  { value: "end", label: "End（末地）" },
  { value: "menu", label: "Menu（菜单音乐）" },
  { value: "overworld_with_menu", label: "Overworld + Menu" },
  { value: "overworld_classic_with_menu", label: "Overworld Classic + Menu" },
];

const normalizeWait = (value: number, fallback: number): number => {
  if (!Number.isFinite(value)) {
    return fallback;
  }
  const normalized = Math.floor(value);
  return normalized > 0 ? normalized : fallback;
};

const getDefaultConf = (): ConfObject => {
  const collection = "overworld";
  const [minWait, maxWait] = getDefaultMinMaxWait(collection);
  return new ConfObject(collection, minWait, maxWait);
};

const loadConf = (): ConfObject => {
  const fallback = getDefaultConf();
  const raw = localStorage.getItem(STORAGE_KEY);
  if (!raw) {
    return fallback;
  }

  try {
    const parsed = JSON.parse(raw) as Partial<ConfObject>;
    const collection = typeof parsed.collection === "string" ? parsed.collection : fallback.collection;
    const [defaultMin, defaultMax] = getDefaultMinMaxWait(collection);
    const minWait = normalizeWait(parsed.minWait ?? defaultMin, defaultMin);
    const maxWait = normalizeWait(parsed.maxWait ?? defaultMax, defaultMax);
    return new ConfObject(collection, Math.min(minWait, maxWait), Math.max(minWait, maxWait));
  } catch {
    return fallback;
  }
};

const conf = reactive<ConfObject>(loadConf());
const isBatchDownloading = ref(false);
const currentDownloadingMusic = ref("");
const autostartEnabled = ref(false);
const autostartLoading = ref(true);
const autostartAvailable = ref(true);

const minWaitSeconds = computed<number>({
  get: () => Math.floor(conf.minWait / SEC),
  set: (value: number) => {
    conf.minWait = normalizeWait(value * SEC, 60 * SEC);
  },
});

const maxWaitSeconds = computed<number>({
  get: () => Math.floor(conf.maxWait / SEC),
  set: (value: number) => {
    conf.maxWait = normalizeWait(value * SEC, 60 * SEC);
  },
});

const persistConf = () => {
  const safeMin = normalizeWait(conf.minWait, 60 * 1000);
  const safeMax = normalizeWait(conf.maxWait, safeMin);
  conf.minWait = Math.min(safeMin, safeMax);
  conf.maxWait = Math.max(safeMin, safeMax);
  localStorage.setItem(STORAGE_KEY, JSON.stringify(new ConfObject(conf.collection, conf.minWait, conf.maxWait)));
};

const onCollectionChange = () => {
  const [nextMin, nextMax] = getDefaultMinMaxWait(conf.collection);
  conf.minWait = nextMin;
  conf.maxWait = nextMax;
  persistConf();
};

const getMusicLabel = (musicPath: string): string => {
  const normalizedPath = musicPath.replaceAll("\\", "/");
  return normalizedPath.split("/").pop() || musicPath;
};

const onDownloadCurrentCollectionMusic = async () => {
  if (isBatchDownloading.value) {
    return;
  }
  isBatchDownloading.value = true;
  currentDownloadingMusic.value = "";
  try {
    const musicPathList = await getMusicFromCollection(conf.collection);
    for (const musicPath of musicPathList) {
      currentDownloadingMusic.value = getMusicLabel(musicPath);
      await requireMusicDownload(musicPath);
    }
  } finally {
    currentDownloadingMusic.value = "";
    isBatchDownloading.value = false;
  }
};

watch(
  () => [conf.collection, conf.minWait, conf.maxWait],
  () => {
    persistConf();
  }
);

onMounted(async () => {
  try {
    autostartEnabled.value = await autostart.isEnabled();
  } catch {
    autostartAvailable.value = false;
  } finally {
    autostartLoading.value = false;
  }
});

const onAutostartChange = async (event: Event) => {
  const input = event.target as HTMLInputElement;
  const next = input.checked;
  try {
    if (next) {
      await autostart.enable();
    } else {
      await autostart.disable();
    }
    autostartEnabled.value = next;
  } catch {
    input.checked = autostartEnabled.value;
  }
};
</script>

<template>
    <div class="page-container-wrapper">
    <section class="page-container settings-page">
    <h2 class="section-title">播放设置</h2>

    <div class="panel">
      <div class="field">
        <label for="collection">Collection</label>
        <select id="collection" v-model="conf.collection" @change="onCollectionChange">
          <option v-for="item in collectionOptions" :key="item.value" :value="item.value">
            {{ item.label }}
          </option>
        </select>
      </div>

      <div class="field">
        <label>缓存当前 Collection 全部音乐</label>
        <button class="download-button" type="button" :disabled="isBatchDownloading" @click="onDownloadCurrentCollectionMusic">
          {{ isBatchDownloading ? "下载中..." : "缓存当前 Collection 全部音乐" }}
        </button>
        <p v-if="isBatchDownloading" class="download-status">
          正在下载：{{ currentDownloadingMusic || "准备中..." }}
        </p>
      </div>

      <div class="field">
        <label class="checkbox-row" :class="{ disabled: !autostartAvailable || autostartLoading }">
          <input
            id="autostart"
            type="checkbox"
            :checked="autostartEnabled"
            :disabled="!autostartAvailable || autostartLoading"
            @change="onAutostartChange"
          />
          <span>开机自启动</span>
        </label>
        <p v-if="!autostartAvailable" class="download-status">仅在 Tauri 桌面环境中可用。</p>
      </div>

      <div class="field-row">
        <div class="field">
          <label for="minWait">minWait（秒）</label>
          <input id="minWait" v-model.number="minWaitSeconds" type="number" min="1" step="1" />
        </div>

        <div class="field">
          <label for="maxWait">maxWait（秒）</label>
          <input id="maxWait" v-model.number="maxWaitSeconds" type="number" min="1" step="1" />
        </div>
      </div>
    </div>

    <h2 class="section-title">关于</h2>
    <div class="panel about">
      <p>Yugen 是一个用于播放 Minecraft 环境音乐的轻量应用，目标是提供专注、自然的听觉氛围。</p>
    </div>
  </section>
    </div>
  
</template>

<style scoped>
.page-container-wrapper {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}
.page-container {
  background: #ffffff;
  border: 1px solid #dcdcdc; /* 经典的边框感 */
  padding: 32px;
  box-sizing: border-box;
}
.settings-page {
  max-width: 860px;
  margin: 0 auto;
}

.section-title {
  margin: 0 0 12px;
  font-size: 18px;
  color: #222;
}

.panel {
  margin-bottom: 28px;
  padding: 18px;
  border: 1px solid #ddd;
  background: #fafafa;
}

.field-row {
  display: grid;
  grid-template-columns: repeat(2, minmax(200px, 1fr));
  gap: 16px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 14px;
}

.field:last-child {
  margin-bottom: 0;
}

label {
  font-size: 14px;
  color: #333;
}

input,
select {
  height: 36px;
  padding: 6px 10px;
  border: 1px solid #bbb;
  font-size: 14px;
  background: #fff;
}

select {
    line-height: 36px;
    padding: 0 10px;
}

.download-button {
  height: 36px;
  padding: 0 12px;
  border: 1px solid #bbb;
  background: #fff;
  font-size: 14px;
  cursor: pointer;
}

.download-button:disabled {
  cursor: not-allowed;
  opacity: 0.7;
}

.download-status {
  margin: 4px 0 0;
  color: #444;
  font-size: 13px;
}

.checkbox-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  user-select: none;
}

.checkbox-row.disabled {
  cursor: not-allowed;
  opacity: 0.65;
}

.checkbox-row input[type="checkbox"] {
  width: 18px;
  height: 18px;
  margin: 0;
  cursor: pointer;
}

.checkbox-row.disabled input[type="checkbox"] {
  cursor: not-allowed;
}

.about p {
  margin: 0 0 10px;
  color: #444;
  line-height: 1.6;
}

.about p:last-child {
  margin-bottom: 0;
}
</style>