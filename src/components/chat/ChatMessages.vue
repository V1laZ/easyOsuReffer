<template>
  <div class="flex-1 overflow-y-auto bg-gray-900 p-4" ref="messagesContainer">
    <!-- Loading State -->
    <div v-if="loading" class="flex items-center justify-center h-32">
      <div class="flex items-center space-x-2 text-gray-400">
        <svg class="animate-spin w-5 h-5" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <span>Loading messages...</span>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else-if="messages.length === 0" class="flex flex-col items-center justify-center h-32 text-gray-500">
      <svg class="w-12 h-12 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
      </svg>
      <p class="text-center">No messages yet</p>
      <p class="text-sm text-gray-600 mt-1">Start chatting or wait for messages to appear</p>
    </div>

    <!-- Messages -->
    <div v-else class="space-y-1">
      <div
        v-for="(message, index) in messages"
        :key="message.id"
        class="group hover:bg-gray-800/50 px-3 py-2 rounded-lg transition-colors"
        :class="{ 'mt-4': shouldShowDateSeparator(message, index) }"
      >
        <!-- Date Separator -->
        <div v-if="shouldShowDateSeparator(message, index)" class="flex items-center justify-center mb-4">
          <div class="flex-1 h-px bg-gray-700"></div>
          <span class="px-4 text-xs text-gray-500 bg-gray-900">
            {{ formatDateSeparator(message.timestamp) }}
          </span>
          <div class="flex-1 h-px bg-gray-700"></div>
        </div>

        <div class="flex items-start space-x-3">
          <!-- Avatar -->
          <div class="flex-shrink-0 mt-1">
            <div class="w-8 h-8 bg-gradient-to-br from-pink-500 to-purple-600 rounded-full flex items-center justify-center">
              <span class="text-sm font-medium text-white">
                {{ message.username.charAt(0).toUpperCase() }}
              </span>
            </div>
          </div>

          <!-- Message Content -->
          <div class="flex-1 min-w-0">
            <!-- Header -->
            <div class="flex items-baseline space-x-2">
              <span 
                class="font-semibold text-white hover:underline cursor-pointer"
                :class="getUsernameColor(message.username)"
                @click="handleUsernameClick(message.username)"
              >
                {{ message.username }}
              </span>
              
              <span class="text-xs text-gray-500">
                {{ formatTime(message.timestamp) }}
              </span>

              <!-- Channel indicator (for PMs or different channels) -->
              <span v-if="message.channel !== activeChannel" class="text-xs text-gray-400">
                in {{ message.channel }}
              </span>
            </div>

            <!-- Message Text -->
            <div class="mt-1">
              <p 
                class="text-white break-words"
                v-html="formatMessageContent(message.message)"
              ></p>
            </div>

            <!-- Message Actions (shown on hover) -->
            <div class="opacity-0 group-hover:opacity-100 transition-opacity mt-1">
              <div class="flex items-center space-x-2">
                <button
                  @click="handleReply(message)"
                  class="text-xs text-gray-500 hover:text-gray-300 transition-colors"
                >
                  Reply
                </button>
                
                <button
                  @click="handleReaction(message)"
                  class="text-xs text-gray-500 hover:text-gray-300 transition-colors"
                >
                  React
                </button>

                <button
                  @click="copyMessage(message.message)"
                  class="text-xs text-gray-500 hover:text-gray-300 transition-colors"
                >
                  Copy
                </button>
              </div>
            </div>
          </div>

          <!-- Message Timestamp (desktop) -->
          <div class="hidden sm:block flex-shrink-0 opacity-0 group-hover:opacity-100 transition-opacity">
            <span class="text-xs text-gray-500">
              {{ formatFullTime(message.timestamp) }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Scroll to bottom indicator -->
    <div 
      v-if="!isAtBottom && messages.length > 0"
      class="fixed bottom-20 right-6 z-10"
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
import { ref, onMounted, onUpdated, nextTick } from 'vue'

interface IrcMessage {
  channel: string
  username: string
  message: string
  timestamp: number
  id: number
}

interface Props {
  messages: IrcMessage[]
  loading: boolean
  activeChannel?: string | null
}

const props = defineProps<Props>()

const messagesContainer = ref<HTMLElement | null>(null)
const isAtBottom = ref(true)

// Username color mapping
const usernameColors = [
  'text-red-400', 'text-green-400', 'text-blue-400', 'text-yellow-400',
  'text-purple-400', 'text-pink-400', 'text-indigo-400', 'text-teal-400'
]

const getUsernameColor = (username: string): string => {
  const hash = username.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0)
  return usernameColors[hash % usernameColors.length]
}

const formatTime = (timestamp: number): string => {
  return new Date(timestamp * 1000).toLocaleTimeString([], { 
    hour: '2-digit', 
    minute: '2-digit' 
  })
}

const formatFullTime = (timestamp: number): string => {
  return new Date(timestamp * 1000).toLocaleTimeString()
}

const formatDateSeparator = (timestamp: number): string => {
  const date = new Date(timestamp * 1000)
  const today = new Date()
  const yesterday = new Date(today.getTime() - 24 * 60 * 60 * 1000)

  if (date.toDateString() === today.toDateString()) {
    return 'Today'
  } else if (date.toDateString() === yesterday.toDateString()) {
    return 'Yesterday'
  } else {
    return date.toLocaleDateString()
  }
}

const shouldShowDateSeparator = (message: IrcMessage, index: number): boolean => {
  if (index === 0) return true
  
  const prevMessage = props.messages[index - 1]
  const currentDate = new Date(message.timestamp * 1000).toDateString()
  const prevDate = new Date(prevMessage.timestamp * 1000).toDateString()
  
  return currentDate !== prevDate
}

const formatMessageContent = (content: string): string => {
  // Basic formatting - in real implementation you'd want more sophisticated parsing
  let formatted = content
  
  // Make URLs clickable
  formatted = formatted.replace(
    /(https?:\/\/[^\s]+)/g,
    '<a href="$1" target="_blank" rel="noopener noreferrer" class="text-blue-400 hover:underline">$1</a>'
  )
  
  // Bold text
  formatted = formatted.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
  
  // Italic text
  formatted = formatted.replace(/\*(.*?)\*/g, '<em>$1</em>')
  
  return formatted
}

const handleUsernameClick = (username: string) => {
  console.log('Username clicked:', username)
  // TODO: Show user profile or start PM
}

const handleReply = (message: IrcMessage) => {
  console.log('Reply to:', message)
  // TODO: Add reply functionality
}

const handleReaction = (message: IrcMessage) => {
  console.log('React to:', message)
  // TODO: Add reaction functionality
}

const copyMessage = async (message: string) => {
  try {
    await navigator.clipboard.writeText(message)
    // TODO: Show toast notification
  } catch (err) {
    console.error('Failed to copy message:', err)
  }
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

onUpdated(() => {
  nextTick(() => {
    if (isAtBottom.value) {
      scrollToBottom()
    }
  })
})
</script>
