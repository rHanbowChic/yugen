<script setup lang="ts">
import { computed, ref } from "vue";
import MainContentPage from "./views/MainContentPage.vue";
import SettingsPage from "./views/SettingsPage.vue";
import player_conf from "./store/player_conf";
import player_state from './store/player_state.ts'

const currentPage = ref("home");

/** 将 player_conf.bgColor（hex）解析为 RGB 分量，供 glow 层使用 */
const glowRgb = computed(() => {
  const hex = player_conf.bgColor;
  // 支持 #RGB 或 #RRGGBB 格式
  return {
    r: parseInt(hex.slice(1, 3), 16),
    g: parseInt(hex.slice(3, 5), 16),
    b: parseInt(hex.slice(5, 7), 16)
  };
});

/*  测试Rust指令是否能正常调用
(async () => {
  const collection = await getMusicFromCollection("underwater")
  for (const item of collection) {
    try {
      await requireMusicDownload(item)
      console.log("下载完成 ", item)
    }
    catch (e) {
      console.log("捕捉到Rust错误", e)
    }

  }
})()
  */

</script>

<template>
  <div class="flex flex-col h-screen w-screen overflow-hidden bg-background text-foreground relative">
    <!-- 背景光晕层 -->
    <div class="absolute inset-0 z-0 pointer-events-none overflow-hidden"
      :style="{
        opacity: player_state.on ? '100%' : '0',
        transition: 'opacity 0.8s ease',
      }">
      <!-- 主光晕 — 颜色跟随 player_conf.bgColor 响应式变化 -->
      <div
        class="glow-orb glow-orb--primary"
        :style="{
          background: `radial-gradient(circle at center, rgba(${glowRgb.r},${glowRgb.g},${glowRgb.b},0.75) 0%, rgba(${glowRgb.r},${glowRgb.g},${glowRgb.b},0.01) 70%, transparent 100%)`,
        }"
      />
      <!-- 辅助光晕 — 同色温但更淡，偏移位置增加层次感 -->
      <div
        class="glow-orb glow-orb--secondary"
        :style="{
          background: `radial-gradient(circle at center, rgba(${glowRgb.r},${glowRgb.g},${glowRgb.b},0.75) 0%, rgba(${glowRgb.r},${glowRgb.g},${glowRgb.b},0.01) 70%, transparent 100%)`,
        }"
      />
    </div>

    <!-- Header -->
    <header
      class="flex items-center justify-between h-14 px-6 bg-zinc-900 text-white shadow-md z-10 shrink-0 relative"
    >
      <div class="flex items-center gap-3">
        <div
          class="size-9 bg-cover bg-center bg-no-repeat rounded-sm border border-white/20"
          style="background-image: url('/image/cherrylog.png')"
        />
        <h1 class="text-lg font-semibold tracking-wide">Yugen</h1>
      </div>

      <nav class="flex h-full">
        <button
          :class="[
            'px-5 h-full text-sm font-medium tracking-wide transition-colors border-0 cursor-pointer',
            currentPage === 'home'
              ? 'text-white bg-zinc-800'
              : 'text-zinc-400 hover:text-white hover:bg-white/5',
          ]"
          @click="currentPage = 'home'"
        >
          主页
        </button>
        <button
          :class="[
            'px-5 h-full text-sm font-medium tracking-wide transition-colors border-0 cursor-pointer',
            currentPage === 'settings'
              ? 'text-white bg-zinc-800'
              : 'text-zinc-400 hover:text-white hover:bg-white/5',
          ]"
          @click="currentPage = 'settings'"
        >
          设置
        </button>
      </nav>
    </header>

    <!-- Main content -->
    <main class="flex-1 relative z-10">
      <MainContentPage :style="{ display: currentPage === 'home' ? undefined : 'none' }" />
      <SettingsPage :style="{ display: currentPage === 'settings' ? undefined : 'none' }" />
    </main>
  </div>
</template>

<style>
/* ---- 背景光晕动画 ---- */
.glow-orb {
  position: absolute;
  border-radius: 50%;
  /* 颜色过渡 — 在 player_conf.bgColor 变化时平滑切换 */
  transition: background 0.8s ease;
}

/* 主光晕：大尺寸，居中偏上 */
.glow-orb--primary {
  width: min(85vw, 85vh);
  height: min(85vw, 85vh);
  top: -15%;
  left: 35%;
  transform: translateX(-50%);
  animation: glow-breathe 8s ease-in-out infinite;
}

/* 辅助光晕：稍小，偏右下，呼吸节奏错开 */
.glow-orb--secondary {
  width: min(75vw, 75vh);
  height: min(75vw, 75vh);
  bottom: 0;
  right: 0;
  animation: glow-breathe 10s ease-in-out infinite 4s;
}

@keyframes glow-breathe {
  0%,
  100% {
    opacity: 0.7;
    scale: 1;
  }
  50% {
    opacity: 1;
    scale: 1.12;
  }
}
</style>
