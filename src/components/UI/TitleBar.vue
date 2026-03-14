<template>
  <div class="flex items-center h-8 bg-gray-900 border-b border-gray-700/50 select-none flex-shrink-0">
    <!-- Draggable region -->
    <div
      data-tauri-drag-region
      class="flex-1 flex items-center px-3 h-full gap-2 cursor-default overflow-hidden"
    >
      <span class="text-xs text-gray-400 font-medium truncate">osu!Reffer</span>
    </div>

    <!-- Window controls -->
    <div class="flex h-full">
      <button
        class="w-10 h-full flex items-center justify-center text-gray-500 hover:text-white hover:bg-gray-700 transition-colors"
        @click="minimize"
      >
        <svg
          width="10"
          height="1"
          viewBox="0 0 10 1"
          fill="currentColor"
        >
          <rect
            width="10"
            height="1"
          />
        </svg>
      </button>

      <button
        class="w-10 h-full flex items-center justify-center text-gray-500 hover:text-white hover:bg-gray-700 transition-colors"
        @click="toggleMaximize"
      >
        <!-- Restore icon -->
        <svg
          v-if="isMaximized"
          width="10"
          height="10"
          viewBox="0 0 10 10"
          fill="none"
          stroke="currentColor"
          stroke-width="1"
        >
          <path d="M2.5 0.5 H9.5 V7.5" />
          <rect
            x="0.5"
            y="2.5"
            width="7"
            height="7"
          />
        </svg>
        <!-- Maximize icon -->
        <svg
          v-else
          width="10"
          height="10"
          viewBox="0 0 10 10"
          fill="none"
          stroke="currentColor"
          stroke-width="1"
        >
          <rect
            x="0.5"
            y="0.5"
            width="9"
            height="9"
          />
        </svg>
      </button>

      <button
        class="w-10 h-full flex items-center justify-center text-gray-500 hover:text-white hover:bg-red-500 transition-colors"
        @click="close"
      >
        <svg
          width="10"
          height="10"
          viewBox="0 0 10 10"
          fill="none"
          stroke="currentColor"
          stroke-width="1.2"
        >
          <line
            x1="0.5"
            y1="0.5"
            x2="9.5"
            y2="9.5"
          />
          <line
            x1="9.5"
            y1="0.5"
            x2="0.5"
            y2="9.5"
          />
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow()
const isMaximized = ref(false)
let unlistenResize: (() => void) | null = null

async function minimize() {
  await appWindow.minimize()
}

async function toggleMaximize() {
  await appWindow.toggleMaximize()
}

async function close() {
  await appWindow.close()
}

onMounted(async () => {
  isMaximized.value = await appWindow.isMaximized()
  unlistenResize = await appWindow.onResized(async () => {
    isMaximized.value = await appWindow.isMaximized()
  })
})

onUnmounted(() => {
  if (unlistenResize) unlistenResize()
})
</script>
