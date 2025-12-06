<template>
  <dialog
    id="invitePlayerDialog"
    ref="invitePlayerDialog"
    class="bg-gray-800 rounded-lg w-full max-w-md"
  >
    <!-- Header -->
    <div class="flex items-center justify-between p-4 border-b border-gray-700">
      <h2 class="text-lg font-semibold text-white">
        Invite Player
      </h2>
      <button
        class="p-2 rounded-lg hover:bg-gray-700 transition-colors"
        @click="invitePlayerDialog?.close()"
      >
        <svg
          class="w-5 h-5 text-gray-400"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M6 18L18 6M6 6l12 12"
          />
        </svg>
      </button>
    </div>

    <!-- Form -->
    <form
      id="invitePlayerDialogForm"
      class="p-4 space-y-4"
      @submit.prevent="handleSubmit()"
    >
      <!-- Player Name Input -->
      <div>
        <label class="block text-sm font-medium text-gray-300 mb-2">Player Name</label>
        <input
          v-model.trim="playerName"
          autofocus
          type="text"
          placeholder="Enter player name"
          maxlength="15"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
        >
      </div>
    </form>

    <!-- Footer -->
    <div class="flex items-center justify-end space-x-3 p-4 border-t border-gray-700">
      <button
        class="px-4 py-2 text-gray-400 hover:text-white transition-colors"
        @click="invitePlayerDialog?.close()"
      >
        Cancel
      </button>
      <button
        form="invitePlayerDialogForm"
        type="submit"
        class="px-6 py-2 bg-pink-600 hover:bg-pink-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg transition-colors"
      >
        Invite
      </button>
    </div>
  </dialog>
</template>

<script setup lang="ts">
import { useDialog } from '@/composables/useDialog'
import { ref, useTemplateRef } from 'vue'

const emit = defineEmits<{
  invite: [playerName: string]
}>()

const playerName = ref<string>('')
const invitePlayerDialog = useTemplateRef('invitePlayerDialog')

useDialog(invitePlayerDialog)

const handleSubmit = () => {
  emit('invite', playerName.value)
  playerName.value = ''
  invitePlayerDialog.value?.close()
}
</script>
