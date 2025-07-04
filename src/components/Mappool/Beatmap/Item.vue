<template>
  <div
    class="group cursor-default relative overflow-hidden bg-gray-800 rounded-xl border border-gray-700 hover:border-gray-600 hover:shadow-lg transition-all duration-300 hover:scale-[1.02]"
  >
    <div class="relative p-4">
      <div class="flex items-start justify-between">
        <div class="flex-1 min-w-0">
          <!-- Category and mod badges -->
          <div class="flex items-center space-x-2 mb-3">
            <span 
              v-if="beatmap.category"
              class="px-3 py-1 text-xs font-bold rounded-full text-white shadow-lg"
              :class="categoryColor"
            >
              {{ beatmap.category }}
            </span>
            <span 
              v-if="beatmap.mod_combination"
              class="px-2 py-1 text-white text-xs font-semibold rounded-full shadow-lg"
            >
              +{{ beatmap.mod_combination }}
            </span>
          </div>
          
          <!-- Song info -->
          <div class="space-y-1">
            <h4 class="text-white font-semibold leading-tight text-base group-hover:text-blue-300 transition-colors">
              {{ beatmap.artist }} - {{ beatmap.title }}
            </h4>
            <p class="text-sm text-gray-300 font-medium">
              [{{ beatmap.difficulty }}] 
              <span class="text-gray-400">by {{ beatmap.mapper }}</span>
            </p>
            <p class="text-xs text-gray-500 font-mono">
              ID: {{ beatmap.beatmap_id }}
            </p>
          </div>
        </div>

        <!-- Remove button -->
        <button 
          @click="emit('remove')"
          class="p-2 text-gray-400 hover:text-red-400 opacity-100 hover:bg-red-900/20 rounded-lg transition-all duration-200 ml-3 sm:opacity-0 group-hover:opacity-100"
          title="Remove from pool"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  beatmap: BeatmapEntry
}>()

const emit = defineEmits<{
  remove: []
}>()

const categoryColor = computed(() => {
  const cat = props.beatmap.category?.slice(0, 2) || '';
  switch (cat) {
    case 'NM': return 'bg-gray-600';
    case 'HD': return 'bg-yellow-600';
    case 'HR': return 'bg-red-600';
    case 'DT': return 'bg-teal-600';
    case 'FL': return 'bg-gray-800';
    case 'FM': return 'bg-purple-600';
    case 'TB': return 'bg-blue-600';
    default: return 'bg-gray-500';
  }
})
</script>