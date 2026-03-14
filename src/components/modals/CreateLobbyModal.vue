<template>
  <AppModal v-model="open">
    <div class="bg-gray-800 rounded-lg w-full max-w-md mx-auto">
      <!-- Header -->
      <div class="flex items-center justify-between p-4 border-b border-gray-700">
        <h2 class="text-lg font-semibold text-white">
          Create Multiplayer Lobby
        </h2>
        <CloseButton @click="open = false" />
      </div>

      <!-- Form -->
      <form
        id="createLobbyForm"
        class="p-4 space-y-4"
        @submit.prevent="handleCreateLobby"
      >
        <!-- Lobby Name Input -->
        <div>
          <label class="block text-sm font-medium text-gray-300 mb-2">Lobby Name</label>
          <input
            ref="lobbyNameInputRef"
            v-model="lobbyName"
            type="text"
            autofocus
            placeholder="Enter lobby name"
            maxlength="50"
            class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
          >
        </div>

        <!-- Team Mode Select -->
        <div>
          <label class="block text-sm font-medium text-gray-300 mb-2">Team Mode</label>
          <div class="relative flex items-center">
            <select
              v-model="teamMode"
              class="appearance-none w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
            >
              <option value="0">
                Head To Head
              </option>
              <option value="1">
                Tag Coop
              </option>
              <option value="2">
                Team Vs
              </option>
              <option value="3">
                Tag Team Vs
              </option>
            </select>
            <span class="select-arrow" />
          </div>
        </div>

        <!-- Score Mode Select -->
        <div>
          <label class="block text-sm font-medium text-gray-300 mb-2">Score Mode</label>
          <div class="relative flex items-center">
            <select
              v-model="scoreMode"
              class="appearance-none w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
            >
              <option value="0">
                Score
              </option>
              <option value="1">
                Accuracy
              </option>
              <option value="2">
                Combo
              </option>
              <option value="3">
                Score V2
              </option>
            </select>
            <span class="select-arrow" />
          </div>
        </div>
      </form>

      <!-- Footer -->
      <div class="flex items-center justify-end space-x-3 p-4 border-t border-gray-700">
        <button
          class="px-4 py-2 text-gray-400 hover:text-white transition-colors"
          @click="open = false"
        >
          Cancel
        </button>
        <button
          form="createLobbyForm"
          type="submit"
          :disabled="!lobbyName.trim() || loading"
          class="px-6 py-2 bg-pink-600 hover:bg-pink-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg transition-colors"
        >
          Create Lobby
        </button>
      </div>
    </div>
  </AppModal>
</template>

<script setup lang="ts">
import { nextTick, ref, useTemplateRef, watch } from 'vue'
import { CreateLobbySettings } from '@/types'
import AppModal from '../UI/AppModal.vue'
import CloseButton from '../UI/CloseButton.vue'

const open = defineModel<boolean>({ required: true })

const emit = defineEmits<{
  createLobby: [settings: CreateLobbySettings]
}>()

const loading = ref(false)
const lobbyName = ref('')
const teamMode = ref<CreateLobbySettings['teamMode']>('2')
const scoreMode = ref<CreateLobbySettings['scoreMode']>('3')
const lobbyNameInputRef = useTemplateRef('lobbyNameInputRef')

const handleCreateLobby = () => {
  const name = lobbyName.value.trim()

  if (!name) {
    alert('Please enter a lobby name')
    return
  }

  if (name.length > 50) {
    alert('Lobby name is too long (max 50 characters)')
    return
  }

  loading.value = true

  emit('createLobby', {
    name: lobbyName.value,
    teamMode: teamMode.value,
    scoreMode: scoreMode.value,
  })
}

watch(open, (newValue) => {
  if (newValue) {
    nextTick(() => {
      console.log('CreateLobbyModal opened, focusing input')
      console.log('lobbyNameInputRef:', lobbyNameInputRef.value)
      lobbyNameInputRef.value?.focus()
    })
  }
}, { immediate: true, flush: 'post' })
</script>
