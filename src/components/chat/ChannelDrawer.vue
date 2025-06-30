<template>
  <div 
    class="fixed inset-y-0 left-0 z-40 w-80 bg-gray-800 border-r border-gray-700 transform transition-transform duration-300 ease-in-out lg:relative lg:translate-x-0"
    :class="isOpen ? 'translate-x-0' : '-translate-x-full'"
  >
    <!-- Header -->
    <div class="flex items-center justify-between p-4 border-b border-gray-700">
      <h2 class="text-lg font-semibold text-white">Channels</h2>
      <button 
        @click="emit('close')"
        class="lg:hidden p-2 rounded-lg hover:bg-gray-700 transition-colors"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Channel List -->
    <div class="flex-1 overflow-y-auto">
      <div 
        v-if="channels.length > 0" 
        class="p-2 space-y-1 mb-4"
      >
        <div v-for="channel in channels" :key="channel" class="relative group">
          <div
            class="flex items-center rounded-lg transition-colors"
            :class="activeChannel === channel 
              ? 'bg-pink-500 text-white' 
              : 'text-gray-300 hover:bg-gray-700 hover:text-white'
            "
          >
            <button
              @click="emit('selectChannel', channel)"
              class="flex-1 flex items-center px-3 py-2 text-left min-w-0"
            >
              <div class="flex items-center space-x-3 flex-1 min-w-0">
                <!-- Channel Icon -->
                <div class="flex-shrink-0">
                  <svg v-if="channel.startsWith('#')" class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M5.41 21L6.12 17H2.12L2.47 15H6.47L7.53 9H3.53L3.88 7H7.88L8.59 3H10.59L9.88 7H15.88L16.59 3H18.59L17.88 7H21.88L21.53 9H17.53L16.47 15H20.47L20.12 17H16.12L15.41 21H13.41L14.12 17H8.12L7.41 21H5.41ZM9.53 9L8.47 15H14.47L15.53 9H9.53Z"/>
                  </svg>
                  <svg v-else class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M12 12C14.21 12 16 10.21 16 8S14.21 4 12 4 8 5.79 8 8 9.79 12 12 12ZM12 14C9.33 14 4 15.34 4 18V20H20V18C20 15.34 14.67 14 12 14Z"/>
                  </svg>
                </div>
                
                <!-- Channel Name -->
                <div class="flex-1 min-w-0">
                  <div class="font-medium truncate">
                    {{ channel }}
                  </div>
                </div>
              </div>
            </button>

            <!-- Leave Channel Button -->
            <button
              @click.stop="handleLeaveChannel(channel)"
              class="flex-shrink-0 m-1 rounded opacity-0 group-hover:opacity-100 transition-all duration-200"
              :class="activeChannel === channel 
                ? 'text-white' 
                : 'text-gray-400 hover:text-white'
              "
              title="Leave channel"
            >
              <svg class="size-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
        </div>
      </div>

      <div v-else class="text-gray-500 p-4 text-sm">
        No channels available.
      </div>

      <!-- Join Channel Section -->
      <div class="p-4 border-t border-gray-700">
        <div class="space-y-3">
          <h3 class="text-sm font-medium text-gray-400 uppercase tracking-wide">Join Channel</h3>
          <div class="flex space-x-2">
            <input
              v-model="newChannelName"
              @keyup.enter="handleJoinChannel"
              type="text"
              placeholder="Channel name or ID"
              class="flex-1 px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
            />
            <button
              @click="handleJoinChannel"
              :disabled="!newChannelName.trim()"
              class="px-3 py-2 bg-pink-500 hover:bg-pink-600 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg transition-colors"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

defineProps<{
  isOpen: boolean
  channels: string[]
  activeChannel: string | null
}>()
const emit = defineEmits<{
  close: [],
  selectChannel: [channel: string],
  joinChannel: [channel: string],
  leaveChannel: [channel: string]
}>()

const newChannelName = ref('')

const handleJoinChannel = () => {
  const channelName = newChannelName.value.trim()
  
  if (!channelName) {
    alert('Please enter a channel name')
    return
  }
  
  // Basic validation for channel name length
  if (channelName.length > 49) { // 49 because we might add # prefix
    alert('Channel name is too long (max 49 characters)')
    return
  }
  
  // Check for obviously invalid characters
  if (channelName.includes(' ')) {
    alert('Channel names cannot contain spaces')
    return
  }
  
  emit('joinChannel', channelName)
  newChannelName.value = ''
}

const handleLeaveChannel = (channel: string) => {
  const confirmMessage = channel.startsWith('#mp_') 
    ? `Are you sure you want to leave the multiplayer room ${channel}? You will stop receiving messages from this room.`
    : `Are you sure you want to leave ${channel}? You will stop receiving messages from this channel.`
  
  if (confirm(confirmMessage)) {
    emit('leaveChannel', channel)
  }
}
</script>
