<template>
  <div 
    class="fixed inset-0 bg-black/80 flex items-center justify-center p-4 z-50" 
    @click="emit('close')"
  >
    <div class="bg-gray-800 rounded-2xl w-full max-w-4xl max-h-[90vh] border border-gray-700 shadow-2xl overflow-hidden" @click.stop>
      <!-- Header -->
      <div class="flex items-center justify-between p-6 border-b border-gray-700">
        <div>
          <h2 class="text-xl font-semibold text-white">Mappools</h2>
          <p class="text-sm text-gray-400 mt-1">Manage your tournament mappools</p>
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
      <div class="h-[calc(90vh-120px)]">
        <!-- Mappool List -->
        <div 
          v-if="!selectedMappool" 
          class="h-full overflow-y-auto"
        >
          <div v-if="!showCreateMappool">
            <ul>
              <li 
                v-for="mappool in mappools"
                :key="mappool.id"
                class="p-4 hover:bg-gray-700 transition-colors cursor-pointer"
                @click="selectMappool(mappool)"
              >
                <h3 class="text-gray-100">{{ mappool.name }}</h3>
                <p class="text-sm text-gray-400">{{ mappool.description }}</p>
              </li>
            </ul>
            <!-- Create Mappool button -->
            <div class="p-4 border-t border-gray-700">
              <button 
                @click="showCreateMappool = true"
                class="w-full bg-pink-600 hover:bg-pink-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
              >
                Create New Mappool
              </button>
            </div>
          </div>

          <!-- Show Create Mappool form -->
          <div v-else class="p-4">
            <h3 class="text-lg font-semibold text-gray-100 mb-4">Create New Mappool</h3>
            <form @submit.prevent="createMappool">
              <div class="mb-4">
                <label class="block text-sm text-gray-300 mb-1">Name</label>
                <input 
                  placeholder="Enter mappool name"
                  v-model="newMappool.name" 
                  type="text" 
                  class="w-full bg-gray-700 border border-gray-600 rounded-lg p-2 text-gray-200 focus:outline-none focus:ring-2 focus:ring-pink-500"
                  required
                />
              </div>
              <div class="mb-4">
                <label class="block text-sm text-gray-300 mb-1">Description</label>
                <textarea 
                  v-model="newMappool.description" 
                  placeholder="Optional description"
                  class="w-full bg-gray-700 border border-gray-600 rounded-lg p-2 text-gray-200 focus:outline-none focus:ring-2 focus:ring-pink-500"
                  rows="3"
                ></textarea>
              </div>
              <div class="flex justify-end space-x-2">
                <button 
                  type="button" 
                  @click="cancelCreate" 
                  class="bg-gray-600 hover:bg-gray-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
                >
                  Cancel
                </button>
                <button 
                  type="submit" 
                  class="bg-pink-600 hover:bg-pink-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
                >
                  Create Mappool
                </button>
              </div>
            </form>
          </div>
        </div>

        <!-- Mappool specific beatmaps -->
        <div v-if="selectedMappool" class="h-full overflow-y-auto">
          <div class="p-4">
            <h3 class="text-lg font-semibold text-gray-100 mb-4">{{ selectedMappool.name }}</h3>
            <p class="text-sm text-gray-400 mb-4">{{ selectedMappool.description }}</p>
            <div v-if="!showAddBeatmap">
              <ul>
                <li 
                  v-for="beatmap in selectedMappoolBeatmaps" 
                  :key="beatmap.beatmap_id"
                  class="p-4 hover:bg-gray-700 transition-colors cursor-pointer flex justify-between items-center"
                >
                  <div>
                    <h4 class="text-gray-100">{{ beatmap.artist }} - {{ beatmap.title }} [{{ beatmap.difficulty }}]</h4>
                    <p class="text-sm text-gray-400">Mapper: {{ beatmap.mapper }}</p>
                  </div>
                  <button 
                    @click.stop="removeBeatmap(beatmap.beatmap_id)"
                    class="text-red-500 hover:text-red-600"
                  >
                    Remove
                  </button>
                </li>
              </ul>
              <button 
                @click="showAddBeatmap = true"
                class="bg-pink-600 w-full hover:bg-pink-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors mb-4"
              >
                Add Beatmap
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { dbService, type Mappool, type BeatmapEntry } from '../../services/database'

const emit = defineEmits<{
  close: []
}>()

const showCreateMappool = ref(false)
const mappools = ref<Mappool[]>([])
const selectedMappool = ref<Mappool | null>(null)
const selectedMappoolBeatmaps = ref<BeatmapEntry[]>([])
const showAddBeatmap = ref(false)

const newMappool = reactive({
  name: '',
  description: ''
})

const newBeatmap = reactive({
  beatmap_id: 0,
  artist: '',
  title: '',
  difficulty: '',
  mapper: '',
  mod_combination: '',
  category: ''
})

const canAddBeatmap = computed(() => {
  return newBeatmap.beatmap_id > 0 && 
         newBeatmap.artist.trim() && 
         newBeatmap.title.trim() && 
         newBeatmap.difficulty.trim() && 
         newBeatmap.mapper.trim()
})

onMounted(async () => {
  await loadMappools()
})

const loadMappools = async () => {
  try {
    mappools.value = await dbService.getMappools()
  } catch (error) {
    console.error('Failed to load mappools:', error)
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

const createMappool = async () => {
  try {
    await dbService.createMappool(newMappool.name, newMappool.description)
    await loadMappools()
    
    cancelCreate()
  } catch (error) {
    console.error('Failed to create mappool:', error)
    alert('Failed to create mappool')
  }
}

const cancelCreate = () => {
  showCreateMappool.value = false
  newMappool.name = ''
  newMappool.description = ''
}

const addBeatmap = async () => {
  if (!selectedMappool.value || !canAddBeatmap.value) return
  
  try {
    await dbService.addBeatmapToPool(
      selectedMappool.value.id!,
      newBeatmap.beatmap_id,
      newBeatmap.artist,
      newBeatmap.title,
      newBeatmap.difficulty,
      newBeatmap.mapper,
      newBeatmap.mod_combination || undefined,
      newBeatmap.category || undefined
    )
    
    // Reload beatmaps for the selected mappool
    await selectMappool(selectedMappool.value)
    cancelAddBeatmap()
  } catch (error) {
    console.error('Failed to add beatmap:', error)
    alert('Failed to add beatmap')
  }
}

const cancelAddBeatmap = () => {
  showAddBeatmap.value = false
  newBeatmap.beatmap_id = 0
  newBeatmap.artist = ''
  newBeatmap.title = ''
  newBeatmap.difficulty = ''
  newBeatmap.mapper = ''
  newBeatmap.mod_combination = ''
  newBeatmap.category = ''
}

const removeBeatmap = async (beatmapId: number) => {
  if (!confirm('Are you sure you want to remove this beatmap from the mappool?')) {
    return
  }
  
  try {
    await dbService.deleteBeatmapFromPool(beatmapId)
    
    // Reload beatmaps for the selected mappool
    if (selectedMappool.value) {
      await selectMappool(selectedMappool.value)
    }
  } catch (error) {
    console.error('Failed to remove beatmap:', error)
    alert('Failed to remove beatmap')
  }
}

const deleteMappool = async (mappoolId: number) => {
  if (!confirm('Are you sure you want to delete this mappool? This action cannot be undone.')) {
    return
  }
  
  try {
    await dbService.deleteMappool(mappoolId)
    await loadMappools()
    
    // Clear selection if the deleted mappool was selected
    if (selectedMappool.value?.id === mappoolId) {
      selectedMappool.value = null
      selectedMappoolBeatmaps.value = []
    }
  } catch (error) {
    console.error('Failed to delete mappool:', error)
    alert('Failed to delete mappool')
  }
}
</script>

<style scoped>
.bg-gray-750 {
  background-color: #374151;
}
</style>
