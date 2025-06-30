<template>
  <div class="flex items-center justify-between p-4 bg-gray-800 border-b border-gray-700">
    <!-- Left Side - Channel Info and Navigation -->
    <div class="flex items-center space-x-4">
      <!-- Mobile Menu Button (Left Drawer) -->
      <button
        @click="$emit('toggle-left-drawer')"
        class="lg:hidden p-2 rounded-lg hover:bg-gray-700 transition-colors"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      </button>

      <!-- Channel Info -->
      <div class="flex items-center space-x-3">
        <!-- Channel Icon -->
        <div class="flex-shrink-0">
          <svg v-if="activeChannel?.startsWith('#')" class="w-6 h-6 text-gray-400" fill="currentColor" viewBox="0 0 24 24">
            <path d="M5.41 21L6.12 17H2.12L2.47 15H6.47L7.53 9H3.53L3.88 7H7.88L8.59 3H10.59L9.88 7H15.88L16.59 3H18.59L17.88 7H21.88L21.53 9H17.53L16.47 15H20.47L20.12 17H16.12L15.41 21H13.41L14.12 17H8.12L7.41 21H5.41ZM9.53 9L8.47 15H14.47L15.53 9H9.53Z"/>
          </svg>
          <svg v-else-if="activeChannel" class="w-6 h-6 text-gray-400" fill="currentColor" viewBox="0 0 24 24">
            <path d="M12 12C14.21 12 16 10.21 16 8S14.21 4 12 4 8 5.79 8 8 9.79 12 12 12ZM12 14C9.33 14 4 15.34 4 18V20H20V18C20 15.34 14.67 14 12 14Z"/>
          </svg>
          <svg v-else class="w-6 h-6 text-gray-500" fill="currentColor" viewBox="0 0 24 24">
            <path d="M12 2C6.48 2 2 6.48 2 12S6.48 22 12 22 22 17.52 22 12 17.52 2 12 2ZM13 17H11V15H13V17ZM13 13H11V7H13V13Z"/>
          </svg>
        </div>

        <!-- Channel Name and Status -->
        <div>
          <h1 class="text-lg font-semibold text-white">
            {{ activeChannel || 'No channel selected' }}
          </h1>
          <div class="flex items-center space-x-2 text-sm text-gray-400">
            <!-- Connection Status -->
            <div class="flex items-center space-x-1">
              <div 
                class="w-2 h-2 rounded-full"
                :class="connectionStatus.connected ? 'bg-green-500' : 'bg-red-500'"
              ></div>
              <span>{{ connectionStatus.connected ? 'Connected' : 'Disconnected' }}</span>
            </div>

            <!-- Channel Topic/Description -->
            <span v-if="activeChannel && getChannelTopic(activeChannel)" class="hidden sm:inline">
              • {{ getChannelTopic(activeChannel) }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Right Side - Action Buttons -->
    <div class="flex items-center space-x-2">
      <!-- Quick Actions (Mobile - collapsed) -->
      <div class="flex items-center space-x-1 sm:hidden">
        <button
          @click="$emit('open-mappools')"
          class="p-2 rounded-lg hover:bg-gray-700 transition-colors"
          title="Mappools"
        >
          <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
          </svg>
        </button>

        <button
          @click="$emit('open-settings')"
          class="p-2 rounded-lg hover:bg-gray-700 transition-colors"
          title="Settings"
        >
          <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
        </button>
      </div>

      <!-- Desktop Actions -->
      <div class="hidden sm:flex items-center space-x-2">
        <button
          @click="$emit('open-mappools')"
          class="flex items-center space-x-2 px-3 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg transition-colors"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
          </svg>
          <span class="hidden md:inline">Mappools</span>
        </button>

        <button
          @click="$emit('open-settings')"
          class="p-2 rounded-lg hover:bg-gray-700 transition-colors"
          title="Settings"
        >
          <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
        </button>
      </div>

      <!-- Mobile Users Button (Right Drawer) -->
      <button
        @click="$emit('toggle-right-drawer')"
        class="lg:hidden p-2 rounded-lg hover:bg-gray-700 transition-colors"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z" />
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
interface ConnectionStatus {
  connected: boolean
  username?: string
}

interface Props {
  activeChannel: string | null
  connectionStatus: ConnectionStatus
}

interface Emits {
  (e: 'toggle-left-drawer'): void
  (e: 'toggle-right-drawer'): void
  (e: 'open-settings'): void
  (e: 'open-mappools'): void
}

defineProps<Props>()
defineEmits<Emits>()

const getChannelTopic = (channel: string): string => {
  // Return topic/description for known channels
  const topics: Record<string, string> = {
    '#osu': 'General discussion about osu!',
    '#lobby': 'Multiplayer matches and tournaments',
    '#help': 'Get help with osu! related questions',
    '#announce': 'Official osu! announcements',
  }
  return topics[channel] || ''
}
</script>
