<template>
  <div
    class="absolute h-[70vh] left-0 z-20 w-full bottom-0 bg-gray-900/80 overflow-hidden shadow-xl shadow-pink-50 border border-gray-700 rounded-t-2xl transform transition-transform ease-in-out duration-500 flex flex-col"
    :class="{
      'translate-y-0': isOpen,
      'translate-y-full': !isOpen,
    }"
    :aria-hidden="!isOpen"
    :inert="!isOpen"
    style="backdrop-filter: blur(8px);"
  >
    <div class="flex items-start justify-between px-6 py-3 bg-gray-800 border-b border-gray-700">
      <h2 class="text-lg font-semibold text-white">
        Select Beatmap
      </h2>
      <button
        class="text-gray-400 hover:text-white text-2xl leading-none"
        @click="$emit('close')"
      >
        &times;
      </button>
    </div>

    <div class="flex-1 overflow-y-auto p-4 sm:p-6">
      <template v-if="!lobbyState.currentMappoolId">
        <div class="flex flex-col items-center justify-center">
          <svg
            class="w-16 h-16 mb-4 text-gray-500"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"
            />
          </svg>
          <p class="text-lg text-gray-200 font-medium mb-2">
            No active mappool selected
          </p>
          <p class="text-gray-400 mb-4">
            Please choose a mappool to continue
          </p>
          <div class="w-full max-w-xs">
            <div class="relative flex items-center">
              <select
                v-model="selectedMappoolId"
                class="appearance-none w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
              >
                <option
                  disabled
                  value="null"
                >
                  Select a mappool
                </option>
                <option
                  v-for="pool in mappools"
                  :key="pool.id"
                  :value="pool.id"
                >
                  {{ pool.name }}
                </option>
              </select>
              <span class="select-arrow" />
            </div>
            <button
              class="mt-4 w-full px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg font-semibold transition-colors disabled:bg-gray-700 disabled:cursor-not-allowed"
              :disabled="!selectedMappoolId"
              @click="setActiveMappool"
            >
              Set as Active
            </button>
          </div>
        </div>
      </template>
      <List
        v-else
        :beatmaps="beatmaps"
        :can-remove="false"
        @select="emit('selectBeatmap', $event)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import List from '../Mappool/Beatmap/List.vue'
import { dbService } from '@/services/database'
import type { LobbyState, BeatmapEntry, Mappool } from '@/types'

const props = defineProps<{
  isOpen: boolean
  lobbyState: LobbyState
  roomId: string
}>()

const emit = defineEmits<{
  close: []
  selectBeatmap: [beatmap: BeatmapEntry]
  setMappool: [mappoolId: number]
}>()

const mappools = ref<Mappool[]>([])
const selectedMappoolId = ref<number | null>(null)
const beatmaps = ref<BeatmapEntry[]>([])

const fetchMappools = async () => {
  try {
    mappools.value = await dbService.getMappools()
  }
  catch (error) {
    mappools.value = []
    console.error('Failed to fetch mappools:', error)
  }
}

const fetchBeatmaps = async (mappoolId: number) => {
  if (!mappoolId) return
  try {
    beatmaps.value = await dbService.getMappoolBeatmaps(mappoolId)
  }
  catch (error) {
    beatmaps.value = []
    console.error('Failed to fetch beatmaps:', error)
  }
}

const setActiveMappool = async () => {
  if (!selectedMappoolId.value || !props.lobbyState) return
  try {
    const res = await invoke<number>('set_mappool', {
      roomId: props.roomId,
      mappoolId: selectedMappoolId.value,
    })
    emit('setMappool', res)
  }
  catch {
    alert('Failed to set mappool')
  }
}

onMounted(() => {
  fetchMappools()
})

watch(() => props.lobbyState.currentMappoolId, (newVal) => {
  if (newVal) {
    fetchBeatmaps(newVal)
    return
  }
  beatmaps.value = []
}, { immediate: true })
</script>
