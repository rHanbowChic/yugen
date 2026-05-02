<script setup lang="ts">
import { ref } from "vue";
import MainContentPage from "./views/MainContentPage.vue"
import SettingsPage from "./views/SettingsPage.vue";

const currentPage = ref('home');

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
  <div class="app-container">
    <header class="app-header">
      <div class="header-left">
        <div class="app-logo">
          <i class="icon-brand"></i>
        </div>
        <h1 class="app-title">Yugen</h1>
      </div>
      
      <nav class="header-nav">
        <button 
          :class="{'nav-item': true, active: currentPage === 'home'}" 
          @click="currentPage = 'home'"
        >
          主页
        </button>
        <button 
          :class="{'nav-item': true, active: currentPage === 'settings'}" 
          @click="currentPage = 'settings'"
        >
          设置
        </button>
      </nav>
    </header>

    <main class="app-main">
      <transition name="fade" mode="out-in">
      <keep-alive>
        <component :is="{ home: MainContentPage, settings: SettingsPage }[currentPage]" />
      </keep-alive>
      </transition>
      
      <!--div class="placeholder-content">
        {{ currentPage === 'home' ? '主页内容区域' : '设置内容区域' }}
      </div-->
    </main>
  </div>
</template>

<style scoped>
/* 基础布局 */
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  font-family: "PingFang SC", "Microsoft YaHei", sans-serif;
  background-color: #f5f5f5; /* 浅灰色背景，衬托内容区 */
  color: #333;
  overflow: hidden;
}

/* 顶栏设计 */
.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 60px;
  padding: 0 24px;
  background-color: #1a1a1a; /* 深邃的灰黑色 */
  color: #ffffff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  z-index: 10;
}

.header-left {
  display: flex;
  align-items: center;
}

.app-logo {
  width: 40px;
  height: 40px;
  background-image: url("/image/cherrylog.png");
  background-color: #4a90e2;
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  margin-right: 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.app-title {
  font-size: 18px;
  font-weight: 600;
  letter-spacing: 1px;
  margin: 0;
}

/* 导航选项 */
.header-nav {
  display: flex;
  height: 100%;
}

.nav-item {
  background: transparent;
  border: none;
  color: #999;
  padding: 0 20px;
  height: 60px;
  cursor: pointer;
  font-size: 14px;
  position: relative;
  /* 严格方角 */
  border-radius: 0; 
}

.nav-item:hover {
  color: #fff;
  background-color: rgba(255, 255, 255, 0.05);
}
.nav-item.active {
  color: #fff;
  background-color: #2c2c2c;
}

/* 底部激活线：微妙的细节 */
.nav-item.active::after {
  content: "";
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 3px;
  background-color: #4a90e2;
}

/* 主内容区域 */
.app-main {
  flex: 1;
  position: relative;
  /* 内部组件容器样式预设 */
}

/* 过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}


</style>