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
      <div
        v-for="(message) in messages"
        :key="message.timestamp"
        class="group hover:bg-gray-800/50 px-3 py-2 rounded-lg transition-colors"
      >
        <div class="flex items-start space-x-3">
          <!-- Avatar -->
          <div class="flex-shrink-0 mt-1">
            <div 
              class="size-8 rounded-full flex items-center justify-center"
              :class="{
                'bg-gray-600': message.username === 'BanchoBot',
                'bg-gradient-to-br from-pink-500 to-purple-600': message.username !== 'BanchoBot'
              }"
            >
              <span class="text-sm font-medium text-white">
                <template v-if="message.username === 'BanchoBot'">
                  <svg  class="size-5 -mt-0.5" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24"><!-- Icon from Material Design Icons by Pictogrammers - https://github.com/Templarian/MaterialDesign/blob/master/LICENSE --><path fill="currentColor" d="M22 14h-1c0-3.87-3.13-7-7-7h-1V5.73A2 2 0 1 0 10 4c0 .74.4 1.39 1 1.73V7h-1c-3.87 0-7 3.13-7 7H2c-.55 0-1 .45-1 1v3c0 .55.45 1 1 1h1v1a2 2 0 0 0 2 2h14c1.11 0 2-.89 2-2v-1h1c.55 0 1-.45 1-1v-3c0-.55-.45-1-1-1M8.68 17.04L7.5 15.86l-1.18 1.18l-1.18-1.18L7.5 13.5l2.36 2.36zm9 0l-1.18-1.18l-1.18 1.18l-1.18-1.18l2.36-2.36l2.36 2.36z"/></svg>
                </template>
                <template v-else>
                  {{ message.username.charAt(0).toUpperCase() }}
                </template>
              </span>
            </div>
          </div>
            

          <!-- Message Content -->
          <div class="flex-1 min-w-0">
            <!-- Header -->
            <div class="flex items-baseline space-x-2">
              <span class="font-semibold text-white hover:underline cursor-pointer">
                {{ message.username }}
              </span>
              
              <span class="text-xs text-gray-500">
                {{ formatTime(message.timestamp) }}
              </span>
            </div>

            <!-- Message Text -->
            <div class="mt-1">
              <p 
                class="text-gray-100 break-words"
                v-html="formatMessageContent(message.message)"
              ></p>
            </div>
          </div>
        </div>
      </div>
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

defineProps<{
  messages: IrcMessage[]
  activeChannel?: string | null
}>()

const messagesContainer = ref<HTMLElement | null>(null)
const isAtBottom = ref(true)

const formatTime = (timestamp: number): string => {
  return new Date(timestamp * 1000).toLocaleTimeString([], { 
    hour: '2-digit', 
    minute: '2-digit' 
  })
}

const formatMessageContent = (content: string): string => {
  let formatted = content
  
  // Make URLs clickable
  formatted = formatted.replace(
    /(https?:\/\/[^\s]+)/g,
    '<a href="$1" target="_blank" rel="noopener noreferrer" class="text-blue-400 hover:underline">$1</a>'
  )
  
  return formatted
}

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
