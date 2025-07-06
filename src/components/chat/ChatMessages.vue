<template>
  <div 
    ref="messagesContainer"
    class="flex-1 overflow-y-auto bg-gray-900 p-4" 
  >
    <!-- Empty State -->
    <div v-if="messages.length === 0" class="flex flex-col items-center justify-center h-32 text-gray-500">
      <svg class="w-12 h-12 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
      </svg>
      <p class="text-center">No messages yet</p>
      <p class="text-sm text-gray-600 mt-1">Start chatting or wait for messages to appear</p>
    </div>

    <!-- Messages -->
    <div v-else class="space-y-1">
      <Message
        v-for="(message, index) in messages"
        :key="`${message.timestamp}${index}`"
        :message="message"
      />
    </div>

    <!-- Scroll to bottom indicator -->
    <div 
      v-if="!isAtBottom && messages.length > 0"
      class="fixed bottom-28 right-6 z-10"
    >
      <button
        @click="scrollToBottom"
        class="flex items-center space-x-2 px-3 py-2 bg-pink-500 hover:bg-pink-600 text-white rounded-full shadow-lg transition-colors"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
        </svg>
        <span class="text-sm">New messages</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUpdated, nextTick, onUnmounted } from 'vue'
import Message from './Message.vue'

defineProps<{
  messages: IrcMessage[]
  activeChannel?: string | null
}>()

const messagesContainer = ref<HTMLElement | null>(null)
const isAtBottom = ref(true)

const scrollToBottom = () => {
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
  }
}

const checkIfAtBottom = () => {
  if (messagesContainer.value) {
    const { scrollTop, scrollHeight, clientHeight } = messagesContainer.value
    isAtBottom.value = scrollTop + clientHeight >= scrollHeight - 5
  }
}

onMounted(() => {
  if (messagesContainer.value) {
    messagesContainer.value.addEventListener('scroll', checkIfAtBottom)
  }
})

onUnmounted(() => {
  if (messagesContainer.value) {
    messagesContainer.value.removeEventListener('scroll', checkIfAtBottom)
  }
})

onUpdated(() => {
  nextTick(() => {
    if (isAtBottom.value) {
      scrollToBottom()
    }
  })
})
</script>
