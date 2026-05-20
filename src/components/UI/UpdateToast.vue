<template>
  <div class="fixed bottom-0 right-0 z-50 w-full max-w-sm p-4">
    <div class="overflow-hidden rounded-xl border border-slate-800 bg-slate-900 shadow-2xl">
      <div
        v-if="!isDownloading && !updateComplete"
        class="p-4"
      >
        <div class="flex items-start gap-3">
          <div class="flex size-10 flex-shrink-0 items-center justify-center rounded-full bg-pink-500/15 text-pink-200 ring-1 ring-inset ring-pink-400/30">
            <Icon
              name="download"
              size="sm"
            />
          </div>
          <div class="min-w-0 flex-1">
            <h3 class="mb-1 font-medium text-slate-100">
              Update available
            </h3>
            <p class="mb-3 text-sm text-slate-400">
              Version {{ updateInfo.latest_version }} is available. You're on {{ updateInfo.current_version }}.
            </p>
            <div class="flex gap-2">
              <Btn
                size="sm"
                block
                @click="installUpdate"
              >
                Update now
              </Btn>
              <Btn
                size="sm"
                variant="ghost"
                @click="emit('close')"
              >
                Later
              </Btn>
            </div>
          </div>
        </div>
      </div>

      <div
        v-else-if="isDownloading"
        class="p-4"
      >
        <div class="flex items-center gap-3">
          <div class="flex size-10 flex-shrink-0 items-center justify-center rounded-full bg-pink-500/15 text-pink-200 ring-1 ring-inset ring-pink-400/30">
            <Spinner class="size-5" />
          </div>
          <div class="min-w-0 flex-1">
            <h3 class="mb-2 font-medium text-slate-100">
              Downloading update
            </h3>
            <div class="mb-2 h-1.5 w-full overflow-hidden rounded-full bg-slate-800">
              <div
                class="h-full bg-pink-400 transition-all duration-300"
                :style="{ width: `${downloadProgress}%` }"
              />
            </div>
            <p class="text-xs text-slate-400">
              {{ downloadProgress.toFixed(0) }}% complete
            </p>
          </div>
        </div>
      </div>

      <div
        v-else-if="updateComplete"
        class="p-4"
      >
        <div class="flex items-start gap-3">
          <div class="flex size-10 flex-shrink-0 items-center justify-center rounded-full bg-emerald-500/15 text-emerald-200 ring-1 ring-inset ring-emerald-400/30">
            <Icon
              name="check"
              size="sm"
            />
          </div>
          <div class="min-w-0 flex-1">
            <h3 class="mb-1 font-medium text-slate-100">
              Update ready
            </h3>
            <p class="mb-3 text-sm text-slate-400">
              The app needs to restart to complete the update.
            </p>
            <div class="flex gap-2">
              <Btn
                size="sm"
                block
                @click="restartApp"
              >
                Restart now
              </Btn>
              <Btn
                size="sm"
                variant="ghost"
                @click="emit('close')"
              >
                Later
              </Btn>
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
import Spinner from './Spinner.vue'
import Btn from './Btn.vue'
import Icon from './Icon.vue'

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
