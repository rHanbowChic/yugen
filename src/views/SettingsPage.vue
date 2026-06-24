<script setup lang="ts">
// 这个组件不能再直接添加新内容了（过于臃肿），请把新内容放在其他组件，在这里调用
import { computed, onMounted, ref } from "vue";
import * as autostart from "@tauri-apps/plugin-autostart";
import { Download } from "@lucide/vue";
import player_conf from "../store/player_conf";
import { COLLECTIONS } from "@/constants";
import { getDefaultMinMaxWait, getMusicFromCollection, requireMusicDownload } from "../utils/utils";
import { Button } from "../components/ui/button";
import { Input } from "../components/ui/input";
import { Label } from "../components/ui/label";
import { Checkbox } from "../components/ui/checkbox";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "../components/ui/select";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../components/ui/card";

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

const isBatchDownloading = ref(false);
const currentDownloadingMusic = ref("");
const autostartEnabled = ref(false);
const autostartLoading = ref(true);
const autostartAvailable = ref(true);

const minWaitSeconds = computed<number>({
  get: () => Math.floor(player_conf.minWait / SEC),
  set: (value: number) => {
    player_conf.minWait = normalizeWait(value * SEC, 60 * SEC);
  },
});

const maxWaitSeconds = computed<number>({
  get: () => Math.floor(player_conf.maxWait / SEC),
  set: (value: number) => {
    player_conf.maxWait = normalizeWait(value * SEC, 60 * SEC);
  },
});

const onCollectionChange = () => {
  const [nextMin, nextMax] = getDefaultMinMaxWait(player_conf.collection);
  player_conf.minWait = COLLECTIONS[player_conf.collection].minWait;
  player_conf.maxWait = COLLECTIONS[player_conf.collection].maxWait;
  player_conf.bgColor = COLLECTIONS[player_conf.collection].bgColor;
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
    const musicPathList = await getMusicFromCollection(player_conf.collection);
    for (const musicPath of musicPathList) {
      currentDownloadingMusic.value = getMusicLabel(musicPath);
      await requireMusicDownload(musicPath);
    }
  } finally {
    currentDownloadingMusic.value = "";
    isBatchDownloading.value = false;
  }
};

onMounted(async () => {
  try {
    autostartEnabled.value = await autostart.isEnabled();
  } catch {
    autostartAvailable.value = false;
  } finally {
    autostartLoading.value = false;
  }
});

const onAutostartChange = async (next: boolean | "indeterminate") => {
  if (next === "indeterminate") return;
  try {
    if (next) {
      await autostart.enable();
    } else {
      await autostart.disable();
    }
    autostartEnabled.value = next;
  } catch {
    // Keep current value on failure
  }
};
</script>

<template>
  <div class="h-full w-full flex flex-col items-center justify-center bg-transparent overflow-auto">
    <div class="w-full max-w-2xl px-6 py-8 space-y-8">
      <!-- 播放设置 -->
      <Card>
        <CardHeader>
          <CardTitle>播放设置</CardTitle>
          <CardDescription>配置音乐播放的相关参数</CardDescription>
        </CardHeader>
        <CardContent class="space-y-6">
          <!-- Collection -->
          <div class="space-y-2">
            <Label for="collection">Collection</Label>
            <Select v-model="player_conf.collection" @update:model-value="onCollectionChange()">
              <SelectTrigger id="collection" class="w-full">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>选择曲库</SelectLabel>
                  <SelectItem
                    v-for="item in collectionOptions"
                    :key="item.value"
                    :value="item.value"
                  >
                    {{ item.label }}
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>

          <!-- 缓存按钮 -->
          <div class="space-y-2">
            <Label>缓存当前 Collection 全部音乐</Label>
            <Button
              variant="outline"
              :disabled="isBatchDownloading"
              @click="onDownloadCurrentCollectionMusic"
            >
              <Download v-if="!isBatchDownloading" class="size-4" />
              {{ isBatchDownloading ? "下载中..." : "缓存当前 Collection 全部音乐" }}
            </Button>
            <p v-if="isBatchDownloading" class="text-sm text-muted-foreground">
              正在下载：{{ currentDownloadingMusic || "准备中..." }}
            </p>
          </div>

          <!-- 开机自启动 -->
          <div class="space-y-2">
            <div
              class="flex items-center gap-3"
              :class="{ 'opacity-65 cursor-not-allowed': !autostartAvailable || autostartLoading }"
            >
              <Checkbox
                id="autostart"
                :model-value="autostartEnabled"
                :disabled="!autostartAvailable || autostartLoading"
                @update:model-value="onAutostartChange"
              />
              <Label
                for="autostart"
                :class="{ 'cursor-not-allowed': !autostartAvailable || autostartLoading }"
              >
                开机自启动
              </Label>
            </div>
            <p v-if="!autostartAvailable" class="text-sm text-muted-foreground">
              仅在 Tauri 桌面环境中可用。
            </p>
          </div>

          <!-- minWait / maxWait -->
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="minWait">minWait（秒）</Label>
              <Input id="minWait" v-model.number="minWaitSeconds" type="number" min="1" step="1" />
            </div>
            <div class="space-y-2">
              <Label for="maxWait">maxWait（秒）</Label>
              <Input id="maxWait" v-model.number="maxWaitSeconds" type="number" min="1" step="1" />
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 关于 -->
      <Card>
        <CardHeader>
          <CardTitle>关于</CardTitle>
        </CardHeader>
        <CardContent>
          <p class="text-muted-foreground leading-relaxed">
            Yugen 是一个用于播放 Minecraft 环境音乐的轻量应用，目标是提供专注、自然的听觉氛围。
          </p>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
