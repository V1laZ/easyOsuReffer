<template>
  <AppModal v-model="open">
    <div class="bg-gray-800 rounded-lg w-full max-w-md mx-auto">
      <!-- Header -->
      <div class="flex items-center justify-between p-4 border-b border-gray-700">
        <h2 class="text-lg font-semibold text-white">
          Invite Player
        </h2>
        <CloseButton @click="open = false" />
      </div>

      <!-- Form -->
      <form
        class="p-4 space-y-4"
        @submit.prevent="handleSubmit"
      >
        <div>
          <label class="block text-sm font-medium text-gray-300 mb-2">Player Name</label>
          <input
            ref="playerNameInputRef"
            v-model.trim="playerName"
            autofocus
            type="text"
            placeholder="Enter player name"
            maxlength="15"
            class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
          >
        </div>

        <!-- Footer -->
        <div class="flex items-center justify-end space-x-3 pt-2 border-t border-gray-700">
          <button
            type="button"
            class="px-4 py-2 text-gray-400 hover:text-white transition-colors"
            @click="open = false"
          >
            Cancel
          </button>
          <button
            type="submit"
            class="px-6 py-2 bg-pink-600 hover:bg-pink-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg transition-colors"
          >
            Invite
          </button>
        </div>
      </form>
    </div>
  </AppModal>
</template>

<script setup lang="ts">
import { ref, useTemplateRef, watch } from 'vue'
import AppModal from '../UI/AppModal.vue'
import CloseButton from '../UI/CloseButton.vue'

const open = defineModel<boolean>({ required: true })

const emit = defineEmits<{
  invite: [playerName: string]
}>()

const playerName = ref('')
const playerNameInputRef = useTemplateRef('playerNameInputRef')

const handleSubmit = () => {
  if (!playerName.value) return
  emit('invite', playerName.value)
  playerName.value = ''
  open.value = false
}

watch(open, (newVal) => {
  if (newVal) {
    playerNameInputRef.value?.focus()
  }
}, { immediate: true, flush: 'post' })
</script>
