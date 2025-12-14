<template>
  <div class="fixed bottom-4 right-4 z-50 w-full shadow max-w-md">
    <div class="bg-gray-800 border border-gray-700 rounded-xl shadow-2xl overflow-hidden">
      <!-- Update Available -->
      <div
        v-if="!isDownloading && !updateComplete"
        class="p-4"
      >
        <div class="flex items-start gap-3">
          <div class="flex-shrink-0 w-10 h-10 rounded-full bg-gradient-to-br from-pink-500 to-purple-600 flex items-center justify-center">
            <svg
              class="w-5 h-5 text-white"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
              />
            </svg>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-white font-medium mb-1">
              Update Available
            </h3>
            <p class="text-sm text-gray-400 mb-3">
              Version {{ updateInfo.latest_version }} is available. You're on {{ updateInfo.current_version }}.
            </p>
            <div class="flex gap-2">
              <button
                class="flex-1 px-3 py-2 bg-gradient-to-br from-pink-500 to-purple-600 hover:from-pink-600 hover:to-purple-700 text-white text-sm font-medium rounded-lg transition-all"
                @click="installUpdate"
              >
                Update Now
              </button>
              <button
                class="px-3 py-2 bg-gray-700 hover:bg-gray-600 text-gray-300 text-sm font-medium rounded-lg transition-colors"
                @click="emit('close')"
              >
                Later
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Downloading -->
      <div
        v-else-if="isDownloading"
        class="p-4"
      >
        <div class="flex items-center gap-3">
          <div class="flex-shrink-0 w-10 h-10 rounded-full bg-gradient-to-br from-pink-500 to-purple-600 flex items-center justify-center">
            <svg
              class="w-5 h-5 text-white animate-spin"
              fill="none"
              viewBox="0 0 24 24"
            >
              <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
              />
              <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              />
            </svg>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-white font-medium mb-1">
              Downloading Update
            </h3>
            <div class="mb-2">
              <div class="w-full h-2 bg-gray-700 rounded-full overflow-hidden">
                <div
                  class="h-full bg-gradient-to-r from-pink-500 to-purple-600 transition-all duration-300"
                  :style="{ width: `${downloadProgress}%` }"
                />
              </div>
            </div>
            <p class="text-sm text-gray-400">
              {{ downloadProgress.toFixed(0) }}% complete
            </p>
          </div>
        </div>
      </div>

      <!-- Update Complete -->
      <div
        v-else-if="updateComplete"
        class="p-4"
      >
        <div class="flex items-start gap-3">
          <div class="flex-shrink-0 w-10 h-10 rounded-full bg-green-500 flex items-center justify-center">
            <svg
              class="w-5 h-5 text-white"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M5 13l4 4L19 7"
              />
            </svg>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-white font-medium mb-1">
              Update Ready
            </h3>
            <p class="text-sm text-gray-400 mb-3">
              The app needs to restart to complete the update.
            </p>
            <div class="flex gap-2">
              <button
                class="flex-1 px-3 py-2 bg-gradient-to-br from-pink-500 to-purple-600 hover:from-pink-600 hover:to-purple-700 text-white text-sm font-medium rounded-lg transition-all"
                @click="restartApp"
              >
                Restart Now
              </button>
              <button
                class="px-3 py-2 bg-gray-700 hover:bg-gray-600 text-gray-300 text-sm font-medium rounded-lg transition-colors"
                @click="emit('close')"
              >
                Later
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { UpdateInfo } from '@/types'
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { relaunch } from '@tauri-apps/plugin-process'

defineProps<{
  updateInfo: UpdateInfo
}>()

const emit = defineEmits<{
  close: []
}>()

const downloadProgress = ref(0)
const isDownloading = ref(false)
const updateComplete = ref(false)

let unsubscribeProgress: UnlistenFn | null = null
let unsubscribeComplete: UnlistenFn | null = null

const installUpdate = async () => {
  try {
    isDownloading.value = true
    downloadProgress.value = 0

    await invoke('install_update')
  }
  catch (error) {
    console.error('Failed to install update:', error)
    emit('close')
  }
}

const restartApp = async () => {
  try {
    await relaunch()
  }
  catch (error) {
    console.error('Failed to restart app:', error)
  }
}

onMounted(async () => {
  unsubscribeProgress = await listen<number>('update-download-progress', (event) => {
    downloadProgress.value = event.payload
  })

  unsubscribeComplete = await listen('update-download-complete', () => {
    isDownloading.value = false
    updateComplete.value = true
  })
})

onUnmounted(() => {
  if (unsubscribeProgress) unsubscribeProgress()
  if (unsubscribeComplete) unsubscribeComplete()
})
</script>
