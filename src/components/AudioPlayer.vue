<template>
  <audio ref="audioRef"></audio>
</template>

<script setup>
import { ref, onBeforeUnmount } from 'vue'

const audioRef = ref(null)

let currentUrl = null
let currentPromise = null
let resolveFn = null
let rejectFn = null

function cleanupUrl() {
  if (currentUrl) {
    URL.revokeObjectURL(currentUrl)
    currentUrl = null
  }
}

function cleanupListeners(audioEl, onEnded, onError) {
  audioEl.removeEventListener('ended', onEnded)
  audioEl.removeEventListener('error', onError)
}

/**
 * 播放
 */
function play(url) {
  // 如果已有播放，先停止
  if (currentPromise) {
    stop()
  }

  const audioEl = audioRef.value
  if (!audioEl) {
    return Promise.reject(new Error('audio element not ready'))
  }

  currentPromise = new Promise((resolve, reject) => {
    resolveFn = resolve
    rejectFn = reject

    cleanupUrl()

    currentUrl = url

    audioEl.src = url

    const onEnded = () => {
      cleanupListeners(audioEl, onEnded, onError)
      cleanupUrl()

      resolveFn?.()
      resetState()
    }

    const onError = (e) => {
      cleanupListeners(audioEl, onEnded, onError)
      cleanupUrl()

      rejectFn?.(e)
      resetState()
    }

    audioEl.addEventListener('ended', onEnded)
    audioEl.addEventListener('error', onError)

    audioEl.play().catch(err => {
      cleanupListeners(audioEl, onEnded, onError)
      reject(err)
      resetState()
    })
  })

  return currentPromise
}

/**
 * 暂停
 */
function pause() {
  const audioEl = audioRef.value
  if (audioEl && !audioEl.paused) {
    audioEl.pause()
  }
}

/**
 * 继续
 */
function resume() {
  const audioEl = audioRef.value
  if (audioEl && audioEl.paused) {
    return audioEl.play()
  }
}

/**
 * 停止（会中断 Promise）
 */
function stop() {
  const audioEl = audioRef.value
  if (!audioEl) return

  audioEl.pause()
  audioEl.currentTime = 0

  cleanupUrl()

  // 中断 await
  if (rejectFn) {
    rejectFn(new Error('stopped'))
  }

  resetState()
}

// 从暂停中唤醒
function wakeup() {
  const audioEl = audioRef.value
  return audioEl.play()
}

function resetState() {
  currentPromise = null
  resolveFn = null
  rejectFn = null
}

onBeforeUnmount(() => {
  stop()
})

defineExpose({
  play,
  pause,
  resume,
  stop,
  wakeup
})
</script>