<script setup lang="ts">
import { ref } from "vue";
import MainContentPage from "./views/MainContentPage.vue";
import SettingsPage from "./views/SettingsPage.vue";

const currentPage = ref("home");

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
  <div class="flex flex-col h-screen w-screen overflow-hidden bg-background text-foreground">
    <!-- Header -->
    <header
      class="flex items-center justify-between h-14 px-6 bg-zinc-900 text-white shadow-md z-10 shrink-0"
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
    <main class="flex-1 relative">
      <MainContentPage :style="{ display: currentPage === 'home' ? undefined : 'none' }" />
      <SettingsPage :style="{ display: currentPage === 'settings' ? undefined : 'none' }" />
    </main>
  </div>
</template>
