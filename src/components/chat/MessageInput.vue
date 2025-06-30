<template>
  <div class="bg-gray-800 border-t border-gray-700 p-4">
    <!-- Message Input Container -->
    <div class="flex-1 relative">
      <!-- Input Field -->
      <div class="relative">
        <input
          v-model="messageText"
          @keydown.enter="sendMessage"
          :disabled="disabled"
          placeholder="Type a message..."
          class="w-full px-4 py-3 pr-12 bg-gray-700 border border-gray-600 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
          :class="{ 'border-red-500': hasError }"
        ></input>

        <!-- Character Count -->
        <div 
          v-if="messageText.length > 400"
          class="absolute bottom-2 right-12 text-xs"
          :class="messageText.length > 500 ? 'text-red-400' : 'text-gray-400'"
        >
          {{ messageText.length }}/500
        </div>

        <!-- Send Button -->
        <button
          @click="sendMessage"
          :disabled="disabled || !canSend"
          class="absolute right-2 top-[0.55rem] p-2 bg-pink-500 hover:bg-pink-600 disabled:bg-gray-700 disabled:cursor-not-allowed text-white rounded-lg transition-colors"
          title="Send message (Ctrl+Enter)"
        >
          <svg class="size-4 rotate-90" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
          </svg>
        </button>
      </div>
    </div>

    <!-- Mobile Keyboard Spacer -->
    <div v-if="keyboardVisible" class="h-4"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

const { disabled = false } = defineProps<{
  disabled?: boolean
}>()

const emit = defineEmits<{
  sendMessage: [message: string]
}>()

const messageText = ref('')
const keyboardVisible = ref(false)
const hasError = ref(false)

const canSend = computed(() => {
  return messageText.value.trim().length > 0 && messageText.value.length <= 500
})

const sendMessage = () => {
  if (!canSend.value || disabled) {
    return
  }

  const message = messageText.value.trim()
  if (!message) {
    return
  }

  emit('sendMessage', message)
  messageText.value = ''
}

// Mobile keyboard detection
const handleResize = () => {
  if (window.innerWidth < 768) {
    const heightDiff = window.innerHeight - document.documentElement.clientHeight
    keyboardVisible.value = heightDiff > 150
  }
}

onMounted(() => {
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
})
</script>
