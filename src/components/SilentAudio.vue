<template>
  <!-- 这个组件不需要任何视觉元素 -->
  <div class="silent-audio-driver" style="display: none;" aria-hidden="true"></div>
</template>

<script setup>
import { onMounted, onUnmounted } from 'vue';

let audioContext = null;
let oscillator = null;

const initAudio = () => {
  const context = new (window.AudioContext || window.webkitAudioContext)();
  
  // 创建一个 1 秒的缓冲区存放随机噪声
  const bufferSize = context.sampleRate;
  const buffer = context.createBuffer(1, bufferSize, context.sampleRate);
  const data = buffer.getChannelData(0);
  
  for (let i = 0; i < bufferSize; i++) {
    // 产生简单的白噪声，通过极低增益后感官类似空气声
    data[i] = Math.random() * 2 - 1;
  }

  const noise = context.createBufferSource();
  noise.buffer = buffer;
  noise.loop = true;

  const gainNode = context.createGain();
  gainNode.gain.value = 0.0005; // 调低音量

  noise.connect(gainNode);
  gainNode.connect(context.destination);
  noise.start();
};

/*
const initAudio = () => {
  const AudioContextClass = window.AudioContext || window.webkitAudioContext;
  if (!AudioContextClass) return;

  audioContext = new AudioContextClass();

  // 创建增益节点（控制音量）
  const gainNode = audioContext.createGain();
  // 设置一个极小但非零的音量，足以维持硬件唤醒但人耳听不到
  gainNode.gain.value = 0.0005;

  // 创建振荡器
  oscillator = audioContext.createOscillator();
  oscillator.type = 'sine'; 
  oscillator.frequency.setValueAtTime(440, audioContext.currentTime);

  // 连接：振荡器 -> 音量控制 -> 扬声器
  oscillator.connect(gainNode);
  gainNode.connect(audioContext.destination);

  oscillator.start();
};
*/

const handleUserInteraction = () => {
  if (!audioContext) {
    initAudio();
  } else if (audioContext.state === 'suspended') {
    // 恢复被浏览器挂起的音频上下文
    audioContext.resume();
  }
  
  // 激活后移除监听，避免重复触发
  window.removeEventListener('click', handleUserInteraction);
  window.removeEventListener('touchstart', handleUserInteraction);
};

onMounted(() => {
  // 监听用户第一次点击或触摸，以绕过浏览器的自动播放限制
  window.addEventListener('click', handleUserInteraction);
  window.addEventListener('touchstart', handleUserInteraction);
});

onUnmounted(() => {
  if (oscillator) {
    oscillator.stop();
    oscillator.disconnect();
  }
  if (audioContext) {
    audioContext.close();
  }
  window.removeEventListener('click', handleUserInteraction);
  window.removeEventListener('touchstart', handleUserInteraction);
});
</script>