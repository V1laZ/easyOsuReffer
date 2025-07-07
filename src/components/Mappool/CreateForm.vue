<template>
  <div class="p-4 border-b border-gray-700 bg-gray-800">
    <h3 class="text-base font-medium text-white mb-3">Create New Mappool</h3>
    <form @submit.prevent="createMappool" class="space-y-3">
      <input 
        v-model="newMappool.name" 
        type="text" 
        placeholder="Mappool name (e.g. OWC 2024 Finals)"
        class="w-full bg-gray-700 border border-gray-600 rounded-lg p-3 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500"
        required
      />
      <textarea 
        v-model="newMappool.description" 
        placeholder="Optional description"
        class="w-full bg-gray-700 border border-gray-600 rounded-lg p-3 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 resize-none"
        rows="2"
      ></textarea>
      <div class="flex gap-2">
        <button 
          type="button" 
          @click="emit('cancel')" 
          class="flex-1 bg-gray-600 hover:bg-gray-700 text-white font-medium py-3 px-4 rounded-lg transition-colors"
        >
          Cancel
        </button>
        <button 
          :disabled="loading"
          type="submit" 
          class="flex-1 bg-pink-600 hover:bg-pink-700 text-white font-medium py-3 px-4 rounded-lg transition-colors"
        >
          Create
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { dbService } from '../../services/database';

const emit = defineEmits<{
  create: [mappool: NewMappoolForm];
  cancel: [];
}>()

const newMappool = ref<NewMappoolForm>({
  name: '',
  description: ''
})
const loading = ref(false)

const createMappool = async () => {
  try {
    loading.value = true
    await dbService.createMappool(newMappool.value.name, newMappool.value.description)
    emit('create', newMappool.value)
  } catch (error) {
    console.error('Failed to create mappool:', error)
    alert('Failed to create mappool')
  } finally {
    loading.value = false
  }
}
</script>