<template>
  <div 
    class="fixed inset-0 bg-black/80 flex items-center justify-center p-4 z-50" 
    @click="emit('close')"
  >
    <div class="bg-gray-900 rounded-2xl w-full max-w-lg md:max-w-4xl max-h-[90vh] border border-gray-700 shadow-2xl overflow-hidden" @click.stop>
      <!-- Header -->
      <div class="flex items-center justify-between p-4 md:p-6 border-b border-gray-700">
        <div>
          <h2 class="text-lg md:text-xl font-semibold text-white">Mappools</h2>
          <p class="text-xs md:text-sm text-gray-400 mt-1">Manage your tournament mappools</p>
        </div>
        <button 
          @click="emit('close')"
          class="p-2 rounded-lg hover:bg-gray-700 transition-colors"
        >
          <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Content -->
      <div class="h-[calc(90vh-180px)] md:h-[calc(90vh-120px)]">
        <!-- Mappool List View -->
        <div v-if="!selectedMappool" class="h-full flex flex-col">
          <CreateForm
            v-if="showCreateForm"
            @create="refreshMappools(); showCreateForm = false"
            @cancel="showCreateForm = false"
          />

          <MappoolList 
            v-else 
            :mappools="mappools"
            @select="selectMappool" 
          />

          <!-- Create Button -->
          <div
            v-if="!showCreateForm" 
            class="p-4 border-t border-gray-700"
          >
            <button 
              class="w-full bg-pink-600 hover:bg-pink-700 text-white font-medium py-3 px-4 rounded-lg transition-colors flex items-center justify-center space-x-2"
              @click="showCreateForm = true"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
              </svg>
              <span>New Mappool</span>
            </button>
          </div>
        </div>

        <!-- Mappool Detail View -->
        <div 
          v-else-if="selectedMappool"
          class="h-full flex flex-col"
        >
          <!-- Header with back button -->
          <div class="p-4 border-b border-gray-700 bg-gray-800">
            <button 
              @click="backToList"
              class="flex items-center space-x-2 text-gray-400 hover:text-white mb-3 transition-colors"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
              </svg>
              <span class="text-sm">Back to mappools</span>
            </button>
            
            <h3 class="text-lg font-medium text-white">{{ selectedMappool?.name }}</h3>
            <p v-if="selectedMappool?.description" class="text-sm text-gray-400 mt-1">{{ selectedMappool.description }}</p>
            
            <div class="flex items-center justify-between mt-3">
              <span class="text-xs text-gray-500">{{ selectedMappoolBeatmaps.length }} maps</span>
              <button 
                @click="deleteMappool(selectedMappool!.id)"
                class="text-xs text-red-400 hover:text-red-300"
              >
                Delete mappool
              </button>
            </div>
          </div>

          <template v-if="!globalState.isConnectedOsu">
            <div class="p-4 text-center text-gray-400">
              <p class="text-sm mb-2">Connect osu! to manage beatmaps</p>
              <ConnectOsuBtn class="mb-auto" />
            </div>
          </template>

          <template v-else>
            <!-- Add Beatmap Form -->
            <AddBeatmapCard 
              v-if="showAddBeatmap"
              :selected-mappool-id="selectedMappool.id"
              @add="refreshBeatmaps(selectedMappool.id); showAddBeatmap = false"
              @cancel="showAddBeatmap = false"
            />
  
            <!-- Beatmap List -->
            <BeatmapList 
              @remove="refreshBeatmaps(selectedMappool.id)"
              :beatmaps="selectedMappoolBeatmaps"
            />
          </template>

          <!-- Add Button -->
          <div class="p-4 border-t border-gray-700">
            <button 
              @click="showAddBeatmap = true"
              :disabled="!globalState.isConnectedOsu"
              class="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-3 px-4 rounded-lg transition-colors flex items-center justify-center space-x-2"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
              </svg>
              <span>{{ globalState.isConnectedOsu ? 'Add Beatmap' : 'Connect osu! to add maps' }}</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { dbService } from '../../services/database'
import { globalState } from '../../stores/global'
import CreateForm from '../Mappool/CreateForm.vue'
import MappoolList from '../Mappool/List.vue'
import AddBeatmapCard from '../Mappool/Beatmap/AddCard.vue'
import BeatmapList from '../Mappool/Beatmap/List.vue'
import ConnectOsuBtn from '../ConnectOsuBtn.vue'

const emit = defineEmits<{
  close: []
}>()

const mappools = ref<Mappool[]>([])
const selectedMappool = ref<Mappool | null>(null)
const selectedMappoolBeatmaps = ref<BeatmapEntry[]>([])
const showCreateForm = ref(false)
const showAddBeatmap = ref(false)

onMounted(async () => {
  await refreshMappools()
})

const refreshMappools = async () => {
  try {
    const loadedMappools = await dbService.getMappools()
    mappools.value = loadedMappools
  } catch (error) {
    console.error('Failed to load mappools:', error)
  }
}

const refreshBeatmaps = async (selectedMappoolId: number) => {
  if (!selectedMappoolId) return
  
  try {
    selectedMappoolBeatmaps.value = await dbService.getMappoolBeatmaps(selectedMappoolId)
  } catch (error) {
    console.error('Failed to refresh beatmaps:', error)
    selectedMappoolBeatmaps.value = []
  }
}

const selectMappool = async (mappool: Mappool) => {
  selectedMappool.value = mappool
  showAddBeatmap.value = false
  
  try {
    selectedMappoolBeatmaps.value = await dbService.getMappoolBeatmaps(mappool.id!)
  } catch (error) {
    console.error('Failed to load mappool beatmaps:', error)
    selectedMappoolBeatmaps.value = []
  }
}

const backToList = () => {
  selectedMappool.value = null
  selectedMappoolBeatmaps.value = []
  showAddBeatmap.value = false
}

const deleteMappool = async (mappoolId: number) => {
  if (!confirm('Are you sure you want to delete this mappool? This action cannot be undone.')) return
  
  try {
    await dbService.deleteMappool(mappoolId)
    await refreshMappools()
    backToList()
  } catch (error) {
    console.error('Failed to delete mappool:', error)
    alert('Failed to delete mappool')
  }
}
</script>
