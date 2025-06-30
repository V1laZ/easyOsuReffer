<template>
  <div 
    class="fixed inset-y-0 right-0 z-40 w-80 bg-gray-800 border-l border-gray-700 transform transition-transform duration-300 ease-in-out lg:relative lg:translate-x-0"
    :class="isOpen ? 'translate-x-0' : 'translate-x-full'"
  >
    <!-- Header -->
    <div class="flex items-center justify-between p-4 border-b border-gray-700">
      <h2 class="text-lg font-semibold text-white">Players</h2>
      <button 
        @click="$emit('close')"
        class="lg:hidden p-2 rounded-lg hover:bg-gray-700 transition-colors"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- User Categories -->
    <div class="flex-1 overflow-y-auto">
      <!-- Online Users -->
      <div class="p-4">
        <div class="flex items-center justify-between mb-3">
          <h3 class="text-sm font-medium text-gray-400 uppercase tracking-wide">
            Online — {{ onlineUsers.length }}
          </h3>
        </div>
        
        <div class="space-y-1">
          <div 
            v-for="user in onlineUsers" 
            :key="user.username"
            class="flex items-center space-x-3 px-2 py-2 rounded-lg hover:bg-gray-700 transition-colors cursor-pointer"
            @click="openUserProfile(user)"
          >
            <!-- Avatar -->
            <div class="relative flex-shrink-0">
              <div class="w-8 h-8 bg-gradient-to-br from-pink-500 to-purple-600 rounded-full flex items-center justify-center">
                <span class="text-sm font-medium text-white">
                  {{ user.username.charAt(0).toUpperCase() }}
                </span>
              </div>
              <!-- Online indicator -->
              <div class="absolute -bottom-0.5 -right-0.5 w-3 h-3 bg-green-500 border-2 border-gray-800 rounded-full"></div>
            </div>

            <!-- User Info -->
            <div class="flex-1 min-w-0">
              <div class="font-medium text-white truncate">
                {{ user.username }}
              </div>
              <div v-if="user.status" class="text-xs text-gray-400 truncate">
                {{ user.status }}
              </div>
            </div>

            <!-- User Actions -->
            <div class="flex-shrink-0">
              <button
                @click.stop="startPrivateMessage(user.username)"
                class="p-1 rounded hover:bg-gray-600 transition-colors"
                title="Send private message"
              >
                <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Offline Users (if any) -->
      <div v-if="offlineUsers.length > 0" class="p-4 border-t border-gray-700">
        <div class="flex items-center justify-between mb-3">
          <h3 class="text-sm font-medium text-gray-400 uppercase tracking-wide">
            Offline — {{ offlineUsers.length }}
          </h3>
          <button 
            @click="showOffline = !showOffline"
            class="p-1 rounded hover:bg-gray-700 transition-colors"
          >
            <svg 
              class="w-4 h-4 text-gray-400 transition-transform"
              :class="showOffline ? 'rotate-180' : ''"
              fill="none" 
              stroke="currentColor" 
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </button>
        </div>
        
        <div v-show="showOffline" class="space-y-1">
          <div 
            v-for="user in offlineUsers" 
            :key="user.username"
            class="flex items-center space-x-3 px-2 py-2 rounded-lg hover:bg-gray-700 transition-colors cursor-pointer opacity-60"
            @click="openUserProfile(user)"
          >
            <!-- Avatar -->
            <div class="relative flex-shrink-0">
              <div class="w-8 h-8 bg-gradient-to-br from-gray-500 to-gray-600 rounded-full flex items-center justify-center">
                <span class="text-sm font-medium text-white">
                  {{ user.username.charAt(0).toUpperCase() }}
                </span>
              </div>
              <!-- Offline indicator -->
              <div class="absolute -bottom-0.5 -right-0.5 w-3 h-3 bg-gray-500 border-2 border-gray-800 rounded-full"></div>
            </div>

            <!-- User Info -->
            <div class="flex-1 min-w-0">
              <div class="font-medium text-gray-300 truncate">
                {{ user.username }}
              </div>
              <div class="text-xs text-gray-500">
                Last seen {{ formatLastSeen(user.lastSeen) }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface User {
  username: string
  online: boolean
  status?: string
  lastSeen?: Date
}

interface Props {
  isOpen: boolean
  users: string[]
}

interface Emits {
  (e: 'close'): void
}

defineProps<Props>()
defineEmits<Emits>()

const showOffline = ref(false)

// Mock user data - in real implementation this would come from props or API
const mockUsers: User[] = [
  { username: 'peppy', online: true, status: 'Playing osu!' },
  { username: 'Cookiezi', online: true, status: 'Streaming' },
  { username: 'WhiteCat', online: false, lastSeen: new Date(Date.now() - 3600000) },
  { username: 'Rafis', online: true },
  { username: 'FlyingTuna', online: false, lastSeen: new Date(Date.now() - 86400000) },
]

const onlineUsers = computed(() => 
  mockUsers.filter(user => user.online)
)

const offlineUsers = computed(() => 
  mockUsers.filter(user => !user.online)
)

const openUserProfile = (user: User) => {
  console.log('Opening profile for:', user.username)
  // TODO: Implement user profile modal
}

const startPrivateMessage = (username: string) => {
  console.log('Starting PM with:', username)
  // TODO: Implement private message functionality
}

const formatLastSeen = (lastSeen?: Date): string => {
  if (!lastSeen) return 'Unknown'
  
  const now = new Date()
  const diff = now.getTime() - lastSeen.getTime()
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)

  if (minutes < 1) return 'Just now'
  if (minutes < 60) return `${minutes}m ago`
  if (hours < 24) return `${hours}h ago`
  return `${days}d ago`
}
</script>
