<template>
  <div class="bg-gray-800 border-t border-gray-700 p-4">
    <div class="flex items-end space-x-3">
      <!-- Attachment Button -->
      <button
        class="flex-shrink-0 p-2 text-gray-400 hover:text-white hover:bg-gray-700 rounded-lg transition-colors"
        title="Attach file"
        @click="handleAttachment"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
        </svg>
      </button>

      <!-- Message Input Container -->
      <div class="flex-1 relative">
        <!-- Input Field -->
        <div class="relative">
          <textarea
            ref="messageInput"
            v-model="messageText"
            @keydown="handleKeyDown"
            @input="handleInput"
            :disabled="disabled"
            :placeholder="getPlaceholder()"
            class="w-full px-4 py-3 pr-12 bg-gray-700 border border-gray-600 rounded-xl text-white placeholder-gray-400 resize-none focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
            :class="{ 'border-red-500': hasError }"
            rows="1"
            style="max-height: 120px; min-height: 48px;"
          ></textarea>

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
            class="absolute bottom-2 right-2 p-2 bg-pink-500 hover:bg-pink-600 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg transition-colors"
            title="Send message (Ctrl+Enter)"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
            </svg>
          </button>
        </div>

        <!-- Typing Indicator -->
        <div v-if="isTyping" class="mt-2 text-xs text-gray-400">
          <span class="inline-flex items-center space-x-1">
            <span>Someone is typing</span>
            <div class="flex space-x-1">
              <div class="w-1 h-1 bg-gray-400 rounded-full animate-pulse"></div>
              <div class="w-1 h-1 bg-gray-400 rounded-full animate-pulse" style="animation-delay: 0.1s;"></div>
              <div class="w-1 h-1 bg-gray-400 rounded-full animate-pulse" style="animation-delay: 0.2s;"></div>
            </div>
          </span>
        </div>

        <!-- Quick Commands -->
        <div v-if="showQuickCommands" class="mt-2">
          <div class="flex flex-wrap gap-2">
            <button
              v-for="command in quickCommands"
              :key="command.text"
              @click="insertCommand(command.text)"
              class="px-2 py-1 bg-gray-700 hover:bg-gray-600 text-xs text-gray-300 rounded transition-colors"
            >
              {{ command.text }}
            </button>
          </div>
        </div>
      </div>

      <!-- Emoji Button -->
      <button
        class="flex-shrink-0 p-2 text-gray-400 hover:text-white hover:bg-gray-700 rounded-lg transition-colors"
        title="Add emoji"
        @click="toggleEmojiPicker"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1m4 0h1m-6 4h1m4 0h1m6-7V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2V9l-6-6z" />
        </svg>
      </button>
    </div>

    <!-- Error Message -->
    <div v-if="errorMessage" class="mt-2 text-sm text-red-400">
      {{ errorMessage }}
    </div>

    <!-- Mobile Keyboard Spacer -->
    <div v-if="keyboardVisible" class="h-4"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick, onMounted, onUnmounted } from 'vue'

interface Props {
  disabled?: boolean
  placeholder?: string
}

interface Emits {
  (e: 'send-message', message: string): void
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
  placeholder: ''
})

const emit = defineEmits<Emits>()

const messageInput = ref<HTMLTextAreaElement | null>(null)
const messageText = ref('')
const isTyping = ref(false)
const showQuickCommands = ref(false)
const keyboardVisible = ref(false)
const hasError = ref(false)
const errorMessage = ref('')

// Quick commands for common IRC/osu! commands
const quickCommands = [
  { text: '!mp start', description: 'Start match' },
  { text: '!mp abort', description: 'Abort match' },
  { text: '!mp map', description: 'Set map' },
  { text: '!mp invite', description: 'Invite player' },
  { text: '!mp kick', description: 'Kick player' },
  { text: '!mp settings', description: 'Show settings' },
]

const canSend = computed(() => {
  return messageText.value.trim().length > 0 && messageText.value.length <= 500
})

const getPlaceholder = (): string => {
  if (props.disabled) {
    return 'Connect to start chatting...'
  }
  return props.placeholder || 'Type a message...'
}

const handleKeyDown = (event: KeyboardEvent) => {
  // Send message on Ctrl+Enter or Enter (mobile)
  if ((event.ctrlKey && event.key === 'Enter') || (event.key === 'Enter' && !event.shiftKey && window.innerWidth < 768)) {
    event.preventDefault()
    sendMessage()
    return
  }

  // Show quick commands on '!'
  if (event.key === '!' && messageText.value === '') {
    showQuickCommands.value = true
  } else if (event.key === 'Escape') {
    showQuickCommands.value = false
  }

  // Auto-resize textarea
  nextTick(() => {
    autoResize()
  })
}

const handleInput = () => {
  clearError()
  autoResize()
  
  // Hide quick commands if text is changed
  if (!messageText.value.startsWith('!')) {
    showQuickCommands.value = false
  }
}

const autoResize = () => {
  if (messageInput.value) {
    messageInput.value.style.height = 'auto'
    messageInput.value.style.height = Math.min(messageInput.value.scrollHeight, 120) + 'px'
  }
}

const sendMessage = () => {
  if (!canSend.value || props.disabled) {
    return
  }

  const message = messageText.value.trim()
  if (!message) {
    return
  }

  try {
    emit('send-message', message)
    messageText.value = ''
    showQuickCommands.value = false
    clearError()
    
    // Reset textarea height
    nextTick(() => {
      if (messageInput.value) {
        messageInput.value.style.height = '48px'
      }
    })
  } catch (error) {
    setError('Failed to send message')
  }
}

const insertCommand = (command: string) => {
  messageText.value = command + ' '
  showQuickCommands.value = false
  
  nextTick(() => {
    if (messageInput.value) {
      messageInput.value.focus()
      messageInput.value.setSelectionRange(messageText.value.length, messageText.value.length)
    }
  })
}

const handleAttachment = () => {
  console.log('Handle attachment')
  // TODO: Implement file attachment
}

const toggleEmojiPicker = () => {
  console.log('Toggle emoji picker')
  // TODO: Implement emoji picker
}

const setError = (message: string) => {
  hasError.value = true
  errorMessage.value = message
  
  setTimeout(() => {
    clearError()
  }, 3000)
}

const clearError = () => {
  hasError.value = false
  errorMessage.value = ''
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
  
  // Focus input when component mounts (desktop only)
  if (window.innerWidth >= 768 && messageInput.value) {
    messageInput.value.focus()
  }
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
})
</script>
