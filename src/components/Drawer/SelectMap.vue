<template>
  <div
    class="absolute bottom-0 left-0 z-20 flex h-[70vh] w-full transform flex-col overflow-hidden rounded-t-xl border border-slate-800 bg-slate-900/95 shadow-2xl backdrop-blur transition-transform duration-300 ease-in-out"
    :class="{
      'translate-y-0': isOpen,
      'translate-y-full': !isOpen,
    }"
    :aria-hidden="!isOpen"
    :inert="!isOpen"
  >
    <header class="flex items-center justify-between border-b border-slate-800 px-5 py-3">
      <h2 class="text-base font-semibold text-slate-100">
        Select beatmap
      </h2>
      <IconBtn
        icon="close"
        size="sm"
        @click="$emit('close')"
      />
    </header>

    <div class="flex-1 overflow-y-auto p-4 sm:p-6">
      <template v-if="!lobbyState.currentMappoolId">
        <div class="flex flex-col items-center justify-center">
          <Icon
            name="musicCollection"
            size="xl"
            class="mb-3 text-slate-600"
          />
          <p class="text-base font-medium text-slate-200">
            No active mappool selected
          </p>
          <p class="mb-5 mt-1 text-sm text-slate-400">
            Choose a mappool to continue
          </p>
          <div class="w-full max-w-xs space-y-3">
            <Select v-model="selectedMappoolId">
              <option
                disabled
                :value="null"
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
            </Select>
            <Btn
              block
              variant="success"
              :disabled="!selectedMappoolId"
              @click="setActiveMappool"
            >
              Set as active
            </Btn>
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
import List from '@/components/Mappool/Beatmap/List.vue'
import { dbService } from '@/services/database'
import type { LobbyState, BeatmapEntry, Mappool } from '@/types'
import IconBtn from '@/components/UI/IconBtn.vue'
import Icon from '@/components/UI/Icon.vue'
import Btn from '@/components/UI/Btn.vue'
import Select from '@/components/UI/Select.vue'

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
