<template>
  <div 
    class="fixed inset-0 bg-black/80 flex items-center justify-center p-4 z-50" 
    @click="$emit('close')"
  >
    <div class="bg-gray-800 rounded-2xl w-full max-w-4xl max-h-[90vh] border border-gray-700 shadow-2xl overflow-hidden" @click.stop>
      <!-- Header -->
      <div class="flex items-center justify-between p-6 border-b border-gray-700">
        <div>
          <h2 class="text-xl font-semibold text-white">Mappools</h2>
          <p class="text-sm text-gray-400 mt-1">Manage your tournament mappools</p>
        </div>
        <button 
          @click="$emit('close')"
          class="p-2 rounded-lg hover:bg-gray-700 transition-colors"
        >
          <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Content -->
      <div class="flex h-[calc(90vh-120px)]">
        <!-- Sidebar - Mappool List -->
        <div class="w-80 border-r border-gray-700 flex flex-col">
          <!-- Create New Mappool -->
          <div class="p-4 border-b border-gray-700">
            <button
              @click="showCreateForm = true"
              class="w-full flex items-center justify-center space-x-2 px-4 py-3 bg-pink-500 hover:bg-pink-600 text-white rounded-lg transition-colors"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
              </svg>
              <span>New Mappool</span>
            </button>
          </div>

          <!-- Mappool List -->
          <div class="flex-1 overflow-y-auto">
            <div v-if="mappools.length === 0" class="p-4 text-center text-gray-500">
              <svg class="w-12 h-12 mx-auto mb-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
              </svg>
              <p>No mappools yet</p>
              <p class="text-sm mt-1">Create your first mappool to get started</p>
            </div>

            <div v-else class="p-2 space-y-1">
              <div
                v-for="mappool in mappools"
                :key="mappool.id"
                @click="selectMappool(mappool)"
                class="p-3 rounded-lg cursor-pointer transition-colors"
                :class="selectedMappool?.id === mappool.id ? 'bg-pink-500 text-white' : 'hover:bg-gray-700 text-gray-300'"
              >
                <div class="font-medium">{{ mappool.name }}</div>
                <div class="text-sm opacity-75 mt-1">
                  {{ mappool.description || 'No description' }}
                </div>
                <div class="text-xs opacity-60 mt-2">
                  {{ getBeatmapCount(mappool.id!) }} maps • {{ formatDate(mappool.created_at) }}
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Main Content -->
        <div class="flex-1 flex flex-col">
          <!-- Create Form -->
          <div v-if="showCreateForm" class="p-6 border-b border-gray-700">
            <h3 class="text-lg font-medium text-white mb-4">Create New Mappool</h3>
            <form @submit.prevent="createMappool" class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">
                  Mappool Name *
                </label>
                <input
                  v-model="newMappool.name"
                  type="text"
                  required
                  class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
                  placeholder="e.g., Summer Tournament 2024 - Semifinals"
                />
              </div>
              
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">
                  Description
                </label>
                <textarea
                  v-model="newMappool.description"
                  rows="3"
                  class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent resize-none"
                  placeholder="Optional description for this mappool..."
                ></textarea>
              </div>

              <div class="flex items-center space-x-3">
                <button
                  type="submit"
                  :disabled="!newMappool.name.trim()"
                  class="px-4 py-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg transition-colors"
                >
                  Create Mappool
                </button>
                
                <button
                  type="button"
                  @click="cancelCreate"
                  class="px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg transition-colors"
                >
                  Cancel
                </button>
              </div>
            </form>
          </div>

          <!-- Selected Mappool Content -->
          <div v-else-if="selectedMappool" class="flex-1 flex flex-col">
            <!-- Mappool Header -->
            <div class="p-6 border-b border-gray-700">
              <div class="flex items-start justify-between">
                <div>
                  <h3 class="text-lg font-medium text-white">{{ selectedMappool.name }}</h3>
                  <p v-if="selectedMappool.description" class="text-gray-400 mt-1">
                    {{ selectedMappool.description }}
                  </p>
                  <div class="text-sm text-gray-500 mt-2">
                    Created {{ formatDate(selectedMappool.created_at) }} • {{ getBeatmapCount(selectedMappool.id!) }} maps
                  </div>
                </div>
                
                <div class="flex items-center space-x-2">
                  <button
                    @click="showAddBeatmap = true"
                    class="px-3 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded-lg transition-colors"
                  >
                    Add Map
                  </button>
                  
                  <button
                    @click="deleteMappool(selectedMappool.id!)"
                    class="px-3 py-2 bg-red-600 hover:bg-red-700 text-white text-sm rounded-lg transition-colors"
                  >
                    Delete Pool
                  </button>
                </div>
              </div>
            </div>

            <!-- Add Beatmap Form -->
            <div v-if="showAddBeatmap" class="p-6 border-b border-gray-700 bg-gray-750">
              <h4 class="text-md font-medium text-white mb-4">Add Beatmap</h4>
              <form @submit.prevent="addBeatmap" class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-300 mb-2">
                    Beatmap ID *
                  </label>
                  <input
                    v-model.number="newBeatmap.beatmap_id"
                    type="number"
                    required
                    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
                    placeholder="123456"
                  />
                </div>

                <div>
                  <label class="block text-sm font-medium text-gray-300 mb-2">
                    Category
                  </label>
                  <select
                    v-model="newBeatmap.category"
                    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
                  >
                    <option value="">Select category</option>
                    <option value="NM">NoMod</option>
                    <option value="HD">Hidden</option>
                    <option value="HR">HardRock</option>
                    <option value="DT">DoubleTime</option>
                    <option value="FM">FreeMod</option>
                    <option value="TB">Tiebreaker</option>
                  </select>
                </div>

                <div>
                  <label class="block text-sm font-medium text-gray-300 mb-2">
                    Artist *
                  </label>
                  <input
                    v-model="newBeatmap.artist"
                    type="text"
                    required
                    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
                    placeholder="Artist name"
                  />
                </div>

                <div>
                  <label class="block text-sm font-medium text-gray-300 mb-2">
                    Title *
                  </label>
                  <input
                    v-model="newBeatmap.title"
                    type="text"
                    required
                    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
                    placeholder="Song title"
                  />
                </div>

                <div>
                  <label class="block text-sm font-medium text-gray-300 mb-2">
                    Difficulty *
                  </label>
                  <input
                    v-model="newBeatmap.difficulty"
                    type="text"
                    required
                    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
                    placeholder="Difficulty name"
                  />
                </div>

                <div>
                  <label class="block text-sm font-medium text-gray-300 mb-2">
                    Mapper *
                  </label>
                  <input
                    v-model="newBeatmap.mapper"
                    type="text"
                    required
                    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
                    placeholder="Mapper name"
                  />
                </div>

                <div class="md:col-span-2">
                  <label class="block text-sm font-medium text-gray-300 mb-2">
                    Mod Combination
                  </label>
                  <input
                    v-model="newBeatmap.mod_combination"
                    type="text"
                    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
                    placeholder="e.g., +HDDT, +HR, etc."
                  />
                </div>

                <div class="md:col-span-2 flex items-center space-x-3">
                  <button
                    type="submit"
                    :disabled="!canAddBeatmap"
                    class="px-4 py-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg transition-colors"
                  >
                    Add Beatmap
                  </button>
                  
                  <button
                    type="button"
                    @click="cancelAddBeatmap"
                    class="px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg transition-colors"
                  >
                    Cancel
                  </button>
                </div>
              </form>
            </div>

            <!-- Beatmap List -->
            <div class="flex-1 overflow-y-auto p-6">
              <div v-if="selectedMappoolBeatmaps.length === 0" class="text-center text-gray-500 py-12">
                <svg class="w-16 h-16 mx-auto mb-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
                </svg>
                <p>No beatmaps in this mappool</p>
                <p class="text-sm mt-1">Add some beatmaps to get started</p>
              </div>

              <div v-else class="space-y-3">
                <div
                  v-for="beatmap in selectedMappoolBeatmaps"
                  :key="beatmap.id"
                  class="p-4 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors"
                >
                  <div class="flex items-start justify-between">
                    <div class="flex-1">
                      <div class="flex items-center space-x-3">
                        <span 
                          v-if="beatmap.category"
                          class="px-2 py-1 bg-pink-500 text-white text-xs font-medium rounded"
                        >
                          {{ beatmap.category }}
                        </span>
                        <span class="text-lg font-medium text-white">
                          {{ beatmap.artist }} - {{ beatmap.title }}
                        </span>
                      </div>
                      
                      <div class="text-gray-300 mt-1">
                        [{{ beatmap.difficulty }}] by {{ beatmap.mapper }}
                      </div>
                      
                      <div class="flex items-center space-x-4 text-sm text-gray-400 mt-2">
                        <span>ID: {{ beatmap.beatmap_id }}</span>
                        <span v-if="beatmap.mod_combination">{{ beatmap.mod_combination }}</span>
                      </div>
                    </div>

                    <button
                      @click="removeBeatmap(beatmap.id!)"
                      class="p-2 text-gray-400 hover:text-red-400 hover:bg-gray-800 rounded transition-colors"
                      title="Remove beatmap"
                    >
                      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                      </svg>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Default State -->
          <div v-else class="flex-1 flex items-center justify-center text-gray-500">
            <div class="text-center">
              <svg class="w-16 h-16 mx-auto mb-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
              </svg>
              <p>Select a mappool to view its contents</p>
              <p class="text-sm mt-1">Or create a new mappool to get started</p>
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

interface Emits {
  (e: 'close'): void
}

defineEmits<Emits>()

const mappools = ref<Mappool[]>([])
const selectedMappool = ref<Mappool | null>(null)
const selectedMappoolBeatmaps = ref<BeatmapEntry[]>([])
const showCreateForm = ref(false)
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
    const id = await dbService.createMappool(newMappool.name, newMappool.description)
    await loadMappools()
    
    // Select the newly created mappool
    const createdMappool = mappools.value.find(m => m.id === id)
    if (createdMappool) {
      await selectMappool(createdMappool)
    }
    
    cancelCreate()
  } catch (error) {
    console.error('Failed to create mappool:', error)
    alert('Failed to create mappool')
  }
}

const cancelCreate = () => {
  showCreateForm.value = false
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

const getBeatmapCount = (_mappoolId: number): number => {
  // This is a simplified count - in a real app you might want to cache this
  return 0 // TODO: Implement proper count
}

const formatDate = (dateString: string): string => {
  return new Date(dateString).toLocaleDateString()
}
</script>

<style scoped>
.bg-gray-750 {
  background-color: #374151;
}
</style>
