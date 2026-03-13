<template>
  <dialog
    id="playerModal"
    ref="dialogRef"
    class="bg-transparent backdrop:bg-black/80 p-0 rounded-2xl w-full max-w-sm border-0 outline-none overflow-hidden"
    @click.self="dialogRef?.close()"
    @close="emit('close')"
    @cancel="dialogRef?.close()"
  >
    <div class="bg-gray-800 border border-gray-700 shadow-2xl rounded-2xl overflow-hidden">
      <!-- Loading -->
      <div
        v-if="loading"
        class="p-8"
      >
        <Loading text="Loading player info…" />
      </div>

      <template v-else>
        <!-- Header with avatar -->
        <div class="relative bg-gray-900 px-6 pt-6 pb-4">
          <CloseButton
            class="absolute top-3 right-3"
            @click="dialogRef?.close()"
          />

          <div class="flex items-center space-x-4">
            <div class="size-16 rounded-full overflow-hidden flex-shrink-0 ring-2 ring-pink-500/50">
              <img
                v-if="player?.avatar_url"
                :src="player.avatar_url"
                :alt="username"
                class="size-full object-cover"
              >
              <div
                v-else
                class="size-full bg-gradient-to-br from-pink-500 to-purple-600 flex items-center justify-center"
              >
                <span class="text-2xl font-bold text-white">{{ username.charAt(0).toUpperCase() }}</span>
              </div>
            </div>

            <div class="min-w-0">
              <h2 class="text-xl font-bold text-white truncate">
                {{ player?.username ?? username }}
              </h2>
              <div
                v-if="player"
                class="flex items-center space-x-1.5 mt-0.5"
              >
                <span class="text-lg leading-none">{{ flagEmoji(player.country) }}</span>
                <span class="text-sm text-gray-400">{{ player.country }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Error -->
        <div
          v-if="error"
          class="px-6 py-4 text-center text-gray-400 text-sm"
        >
          {{ error }}
        </div>

        <!-- Stats -->
        <div
          v-if="player"
          class="px-6 py-4 grid grid-cols-2 gap-3"
        >
          <div class="bg-gray-700/50 rounded-xl p-3">
            <div class="text-xs text-gray-400 mb-0.5">
              Global Rank
            </div>
            <div class="text-white font-semibold">
              {{ player.rank != null ? `#${player.rank.toLocaleString()}` : '—' }}
            </div>
          </div>
          <div class="bg-gray-700/50 rounded-xl p-3">
            <div class="text-xs text-gray-400 mb-0.5">
              Country Rank
            </div>
            <div class="text-white font-semibold">
              {{ player.country_rank != null ? `#${player.country_rank.toLocaleString()}` : '—' }}
            </div>
          </div>
          <div class="bg-gray-700/50 rounded-xl p-3">
            <div class="text-xs text-gray-400 mb-0.5">
              Performance
            </div>
            <div class="text-white font-semibold">
              {{ Math.round(player.pp).toLocaleString() }}pp
            </div>
          </div>
          <div class="bg-gray-700/50 rounded-xl p-3">
            <div class="text-xs text-gray-400 mb-0.5">
              Accuracy
            </div>
            <div class="text-white font-semibold">
              {{ player.accuracy.toFixed(2) }}%
            </div>
          </div>
        </div>

        <!-- Footer -->
        <div class="px-6 pb-5 flex justify-end">
          <button
            class="flex items-center space-x-1.5 px-3 py-2 text-sm text-gray-400 hover:text-white hover:bg-gray-700 rounded-lg transition-colors"
            @click="openInBrowser"
          >
            <span>Open in browser</span>
            <svg
              class="w-4 h-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
              />
            </svg>
          </button>
        </div>
      </template>
    </div>
  </dialog>
</template>

<script setup lang="ts">
import { ref, onMounted, useTemplateRef } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { openUrl } from '@tauri-apps/plugin-opener'
import { useDialog } from '@/composables/useDialog'
import { dbService } from '@/services/database'
import { globalState } from '@/stores/global'
import type { UserData } from '@/types'
import CloseButton from '@/components/UI/CloseButton.vue'
import Loading from '@/components/UI/Loading.vue'

const props = defineProps<{
  username: string
}>()

const emit = defineEmits<{
  close: []
}>()

const dialogRef = useTemplateRef('dialogRef')
useDialog(dialogRef)

const loading = ref(true)
const player = ref<UserData | null>(null)
const error = ref<string | null>(null)

const flagEmoji = (code: string) =>
  code.toUpperCase().split('').map(c => String.fromCodePoint(0x1F1E6 + c.charCodeAt(0) - 65)).join('')

const openInBrowser = () =>
  openUrl(`https://osu.ppy.sh/users/${encodeURIComponent(props.username)}`)

onMounted(async () => {
  dialogRef.value?.showModal()

  try {
    const accessToken = await dbService.getAccessToken(globalState.user ?? '')
    if (!accessToken) {
      error.value = 'Connect your osu! account in Settings to view player info.'
      return
    }

    player.value = await invoke<UserData>('fetch_user_data', {
      username: props.username,
      accessToken,
    })
  }
  catch (err) {
    error.value = 'Could not load player info.'
    console.error('Failed to fetch player data:', err)
  }
  finally {
    loading.value = false
  }
})
</script>
