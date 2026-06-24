<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { Play, Pause } from "@lucide/vue";
import AudioPlayer from "../components/AudioPlayer.vue";
import player_conf from "../store/player_conf";
import player_state from "../store/player_state"
import { getMusicFromCollection, requireMusicDownload } from "../utils/utils";
import { getMatches } from "@tauri-apps/plugin-cli";
import { listen } from "@tauri-apps/api/event";

const playerRef = ref<any>(null);
const isMusicPlaying = ref(false);

const currentCollection = computed(() => player_conf.collection);

const labelText = computed(() =>
  player_state.on ? player_state.path : `Collection: ${currentCollection.value}`
);

let playSessionId = 0;
let pendingTimer: number | null = null;

const randomInt = (min: number, max: number): number => {
  if (max <= min) {
    return min;
  }
  return Math.floor(Math.random() * (max - min + 1)) + min;
};

const sleepWithCancel = (ms: number, sessionId: number): Promise<void> =>
  new Promise((resolve) => {
    pendingTimer = window.setTimeout(() => {
      if (playSessionId === sessionId) {
        resolve();
      }
    }, ms);
  });

const playLoop = async (sessionId: number) => {
  while (playSessionId === sessionId) {
    const minWait = Math.min(player_conf.minWait, player_conf.maxWait);
    const maxWait = Math.max(player_conf.minWait, player_conf.maxWait);
    const musicList = await getMusicFromCollection(player_conf.collection);

    if (playSessionId !== sessionId || !musicList.length) {
      player_state.on = false;
      player_state.path = musicList.length ? player_state.path : "当前 Collection 没有可播放曲目";
      return;
    }

    const nextPath = musicList[randomInt(0, musicList.length - 1)];
    player_state.path = nextPath;

    await requireMusicDownload(nextPath);

    document.title = nextPath;

    if (playSessionId !== sessionId) {
      return;
    }

    if (playSessionId !== sessionId) {
      return;
    }

    isMusicPlaying.value = true;
    const fullPath = "http://127.0.0.1:10454/assets/" + nextPath;
    try {
      await playerRef.value?.play(fullPath);
    } catch (e) {
      return;
    }
    isMusicPlaying.value = false;

    if (playSessionId !== sessionId) {
      return;
    }

    player_state.path = "...";
    document.title = "...";
    const waitMs = randomInt(minWait, maxWait);
    await sleepWithCancel(waitMs, sessionId);
  }
};

// 看门狗 -- 在audio元素意外暂停时恢复（如：设备睡眠醒来时）
setInterval(() => {
  if (isMusicPlaying.value) {
    playerRef.value?.wakeup();
  }
}, 3000);

const stopPlayback = () => {
  playSessionId += 1;
  player_state.on = false;
  isMusicPlaying.value = false;
  playerRef.value?.stop();

  if (pendingTimer !== null) {
    clearTimeout(pendingTimer);
    pendingTimer = null;
  }
};

const startPlayback = async () => {
  player_state.on = true;
  const sessionId = playSessionId + 1;
  playSessionId = sessionId;

  try {
    await playLoop(sessionId);
  } catch (error) {
    console.error("播放循环失败:", error);
    stopPlayback();
  }
};

const togglePlay = () => {
  if (player_state.on) {
    stopPlayback();
    return;
  }
  void startPlayback();
};

let removeTogglePlayListener: Function;

onMounted(async () => {
  // 处理命令行参数
  const matches = await getMatches();
  const args = matches.args;
  if (args.silent?.value) {
    if (!player_state.on) {
      togglePlay();
    }
  }
  // 响应托盘菜单请求
  removeTogglePlayListener = await listen("toggle-play", () => {
    togglePlay();
  });
});

onUnmounted(() => {
  removeTogglePlayListener?.();
});
</script>

<template>
  <div class="h-full w-full flex flex-col justify-around items-center bg-transparent">
    <!-- Music label -->
    <p class="text-2xl text-muted-foreground text-center px-4">{{ labelText }}</p>

    <!-- Play / Pause button -->
    <button
      class="size-20 rounded-full flex items-center justify-center transition-all duration-300 cursor-pointer border-0 outline-none"
      :class="
        player_state.on
          ? 'bg-primary/70 hover:bg-primary/60 active:scale-95'
          : 'bg-primary/90 hover:bg-primary/85 active:scale-95'
      "
      @click="togglePlay"
      :aria-label="player_state.on ? '暂停' : '播放'"
    >
      <Transition name="fade" mode="out-in">
        <Play v-if="!player_state.on" key="play" class="size-8 text-primary-foreground" />
        <Pause v-else key="pause" class="size-8 text-primary-foreground" />
      </Transition>
    </button>

    <AudioPlayer ref="playerRef" />
  </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: scale(0.8);
}
</style>
