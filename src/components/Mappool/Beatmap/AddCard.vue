<template>
  <div class="p-4 border-b border-gray-700 bg-gray-800">
    <div class="space-y-3">
      <div v-if="!beatmapPreview" class="flex items-center space-x-2">
        <input 
          v-model="beatmapInput" 
          type="text" 
          placeholder="Beatmap ID or osu.ppy.sh link"
          class="flex-1 bg-gray-700 border border-gray-600 rounded-lg p-3 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500"
          @keyup.enter="fetchBeatmapData"
        />
        <button 
          @click="fetchBeatmapData"
          :disabled="!beatmapInput || isLoading"
          class="px-4 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white rounded-lg transition-colors"
        >
          <svg v-if="isLoading" class="animate-spin w-4 h-4" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <span v-else>Fetch</span>
        </button>
      </div>

      <!-- Beatmap Preview -->
      <div v-if="beatmapPreview" class="p-4 bg-gray-700 rounded-xl border border-gray-600 backdrop-blur-sm">
        <div class="flex items-center space-x-3 mb-4">
          <div class="relative">
            <img 
              :src="`https://assets.ppy.sh/beatmaps/${beatmapPreview.beatmapset_id}/covers/cover.jpg`" 
              :alt="beatmapPreview.title"
              class="w-16 h-16 rounded-lg object-cover shadow-lg"
            />
            <div class="absolute inset-0 bg-gradient-to-br from-transparent to-black/20 rounded-lg"></div>
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-white font-semibold truncate text-base">{{ beatmapPreview.artist }} - {{ beatmapPreview.title }}</p>
            <p class="text-sm text-gray-300 truncate">[{{ beatmapPreview.difficulty }}] by {{ beatmapPreview.mapper }}</p>
            <div class="flex items-center space-x-3 mt-1">
              <span class="text-xs text-gray-400">★{{ beatmapPreview.difficulty_rating?.toFixed(2) }}</span>
              <span class="text-xs text-gray-400">{{ Math.floor(beatmapPreview.total_length / 60) }}:{{ String(beatmapPreview.total_length % 60).padStart(2, '0') }}</span>
              <span class="text-xs text-gray-400">{{ beatmapPreview.bpm }}&nbsp;BPM</span>
            </div>
          </div>
        </div>
        
        <!-- Category and Mod Selection -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3 mb-4">
          <div>
            <label class="block text-sm font-medium text-gray-300 mb-2">Category *</label>
            <input 
              v-model="beatmapMeta.category"
              type="text" 
              placeholder="e.g. NM2, HD1"
              class="w-full bg-gray-600 border border-gray-500 rounded-lg p-3 text-white text-sm placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-300 mb-2">Mod Combination</label>
            <div class="flex items-center gap-2 flex-wrap">
              <Mod 
                v-for="mod in mods" 
                :key="mod"
                class="cursor-pointer"
                :mod="mod"
                :active="computedMods.includes(mod)"
                @click="handleModSelect(mod)"
              />
            </div>
          </div>
        </div>

        <div class="flex gap-3">
          <button 
            @click="emit('cancel')"
            class="bg-gray-600 w-full hover:bg-gray-700 text-white text-sm font-medium py-3 px-4 rounded-lg transition-colors"
          >
            Cancel
          </button>
          <button 
            @click="addBeatmap"
            :disabled="!beatmapMeta.category"
            class="w-full disabled:bg-gray-600 bg-green-500 hover:bg-green-600 text-white text-sm font-medium py-3 px-4 rounded-lg transition-colors"
          >
            Add to Pool
          </button>
        </div>
      </div>

      <!-- Error State -->
      <div v-if="fetchError" class="p-4 bg-red-900/30 border border-red-800 rounded-lg">
        <div class="flex items-center space-x-2">
          <svg class="w-5 h-5 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <p class="text-red-200 text-sm">{{ fetchError }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { dbService } from '../../../services/database';
import { globalState } from '../../../stores/global';
import Mod from '../../Mod.vue';

const props = defineProps<{
  selectedMappoolId: number
}>()

const emit = defineEmits<{
  cancel: []
  add: []
}>()

const mods = ['NF', 'HD', 'HR', 'DT', 'EZ', 'FL', 'HT', 'FM']
const selectedMods = ref<string[]>(['NF']);
const beatmapPreview = ref<BeatmapData | null>(null);
const beatmapInput = ref('');
const beatmapMeta = ref<{ category: string; mod_combination: string }>({ category: '', mod_combination: '' });
const isLoading = ref(false);
const fetchError = ref('');

const computedMods = computed(() => {
  if (selectedMods.value.includes('FM')) return ['FM'];
  return selectedMods.value
})

const fetchBeatmapData = async () => {
  if (!globalState.userId) return
  
  const beatmapId = extractBeatmapId(beatmapInput.value)
  if (!beatmapId) {
    fetchError.value = 'Invalid beatmap ID or URL format'
    return
  }

  isLoading.value = true
  fetchError.value = ''

  
  try {
    const accessToken = await dbService.getAccessToken(globalState.userId)
    const data = await invoke<BeatmapData>('fetch_beatmap_data', {
      beatmapId: beatmapId,
      accessToken
    })
    
    beatmapPreview.value = data
  } catch (error) {
    console.error('Failed to fetch beatmap data:', error)
    fetchError.value = error instanceof Error ? error.message : 'Failed to fetch beatmap data'
  } finally {
    isLoading.value = false
  }
}

const handleModSelect = (mod: string) => {
  if (selectedMods.value.includes(mod)) {
    selectedMods.value = selectedMods.value.filter(m => m !== mod)
    return
  } 
  
  selectedMods.value.push(mod)

  switch (mod) {
    case 'DT':
      selectedMods.value = selectedMods.value.filter(m => m !== 'HT')
      break
    case 'HT':
      selectedMods.value = selectedMods.value.filter(m => m !== 'DT')
      break
    case 'HR':
      selectedMods.value = selectedMods.value.filter(m => m !== 'EZ')
      break
    case 'EZ':
      selectedMods.value = selectedMods.value.filter(m => m !== 'HR')
      break
  }
}

const extractBeatmapId = (input: string): string | null => {
  const urlPatterns = [
    /osu\.ppy\.sh\/beatmapsets\/\d+#\w+\/(\d+)/,
    /osu\.ppy\.sh\/b\/(\d+)/,
    /osu\.ppy\.sh\/beatmaps\/(\d+)/
  ]
  
  for (const pattern of urlPatterns) {
    const match = input.match(pattern)
    if (match) return match[1]
  }
  
  // Check if it's just a number
  if (/^\d+$/.test(input.trim())) {
    return input.trim()
  }
  
  return null
}

const addBeatmap = async () => {
  if (!beatmapPreview.value || !beatmapMeta.value.category) return

  const category = beatmapMeta.value.category.trim().toUpperCase()
  const modCombination = computedMods.value.join('')

  try {
    await dbService.addBeatmapToPool(
      props.selectedMappoolId,
      beatmapPreview.value.id,
      beatmapPreview.value.artist,
      beatmapPreview.value.title,
      beatmapPreview.value.difficulty,
      beatmapPreview.value.mapper,
      modCombination,
      category,
    )
    emit('add')
  } catch (error) {
    console.error('Failed to add beatmap to pool:', error)
    fetchError.value = error instanceof Error ? error.message : 'Failed to add beatmap to pool'
  }
}
</script>