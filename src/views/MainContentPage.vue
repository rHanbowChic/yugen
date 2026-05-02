<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import AudioPlayer from "../components/AudioPlayer.vue";
import player_conf from "../store/player_conf";
import { getMusicFromCollection, requireMusicDownload } from "../utils/utils";
import { getMatches } from "@tauri-apps/plugin-cli";

const isPlaying = ref(false);
const currentMusicPath = ref("未开始播放");
const playerRef = ref<any>(null);
const isMusicPlaying = ref(false)

const currentCollection = computed(() => player_conf.collection)

const labelText = computed(() => (isPlaying.value ? currentMusicPath.value : `Collection: ${currentCollection.value}`));

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
      isPlaying.value = false;
      currentMusicPath.value = musicList.length ? currentMusicPath.value : "当前 Collection 没有可播放曲目";
      return;
    }

    const nextPath = musicList[randomInt(0, musicList.length - 1)];
    currentMusicPath.value = nextPath;

    await requireMusicDownload(nextPath);

    document.title = nextPath;

    if (playSessionId !== sessionId) {
      return;
    }

    if (playSessionId !== sessionId) {
      return;
    }

    isMusicPlaying.value = true;
    const fullPath = "http://127.0.0.1:10454/assets/" + nextPath
    try {
      await playerRef.value?.play(fullPath)
    }
    catch (e) {
      return;
    }
    isMusicPlaying.value = false;
    

    if (playSessionId !== sessionId) {
      return;
    }

    currentMusicPath.value = "..."
    document.title = "..."
    const waitMs = randomInt(minWait, maxWait);
    await sleepWithCancel(waitMs, sessionId);
  }
};

// 看门狗 -- 在audio元素意外暂停时恢复（如：设备睡眠醒来时）
setInterval(() => {
  if (isMusicPlaying.value) {
    playerRef.value?.wakeup();
  }
}, 3000)

const stopPlayback = () => {
  playSessionId += 1;
  isPlaying.value = false;
  isMusicPlaying.value = false;
  playerRef.value?.stop();

  if (pendingTimer !== null) {
    clearTimeout(pendingTimer);
    pendingTimer = null;
  }
};

const startPlayback = async () => {
  isPlaying.value = true;
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
  if (isPlaying.value) {
    stopPlayback();
    return;
  }
  void startPlayback();
};

onMounted(async () => {
  const matches = await getMatches();
  const args = matches.args;
  if (args.silent?.value) {
    if(!isPlaying.value) {
      togglePlay();
    }
  }
})

</script>

<template>
  <div class="page-container">
    <div class="label">{{ labelText }}</div>
    <button class="player-button" @click="togglePlay" :aria-label="isPlaying ? '暂停' : '播放'">
      <transition name="fade" mode="out-in">
        <svg v-if="!isPlaying" key="play" viewBox="0 0 24 24" class="icon">
          <path d="M8 5v14l11-7z" fill="currentColor" />
        </svg>
        <svg v-else key="pause" viewBox="0 0 24 24" class="icon">
          <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z" fill="currentColor" />
        </svg>
      </transition>
    </button>
    <AudioPlayer ref="playerRef" />

  </div>
</template>

<style scoped>
.page-container {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  align-items: center;
  background-color: #f5f5f5;
}

.player-button {
  width: 80px;
  height: 80px;
  background-color: rgba(60, 119, 185, 0.9);
  border: 1px solid rgba(60, 119, 185, 0.8);
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  justify-content: center;
  align-items: center;
  color: #ffffff;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(8px); /* 磨砂玻璃效果 */
  outline: none;
}

.player-button:hover {
  background-color: rgba(60, 119, 185, 0.85);
  transform: scale(1.05);
  border-color: rgba(60, 119, 185, 0.6);
}

.player-button:active {
  transform: scale(0.95);
  background-color: rgba(60, 119, 185, 0.15);
}

.icon {
  width: 40px;
  height: 40px;
  /* 视觉修正：播放图标稍微向右偏移一点，使其在视觉上更居中 */
}

/* 切换动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: scale(0.8);
}

.label {
  font-size: 24px;
}
</style>