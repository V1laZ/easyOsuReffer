<template>
  <div class="relative group">
    <div
      class="flex items-center rounded-lg transition-colors"
      :class="isActive 
        ? 'bg-pink-500 text-white' 
        : 'text-gray-300 hover:bg-gray-700 hover:text-white'
      "
    >
      <button
        @click="emit('select', room.id)"
        class="flex-1 flex items-center px-3 py-2 text-left min-w-0"
      >
        <div class="flex items-center space-x-3 flex-1 min-w-0">
          <!-- Room Icon -->
          <div class="flex-shrink-0">
            <svg v-if="room.roomType === 'Channel'" class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
              <path d="M5.41 21L6.12 17H2.12L2.47 15H6.47L7.53 9H3.53L3.88 7H7.88L8.59 3H10.59L9.88 7H15.88L16.59 3H18.59L17.88 7H21.88L21.53 9H17.53L16.47 15H20.47L20.12 17H16.12L15.41 21H13.41L14.12 17H8.12L7.41 21H5.41ZM9.53 9L8.47 15H14.47L15.53 9H9.53Z"/>
            </svg>
            <svg v-else-if="room.roomType === 'MultiplayerLobby'" class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2M17,13H13V17H11V13H7V11H11V7H13V11H17V13Z"/>
            </svg>
            <svg v-else class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 12C14.21 12 16 10.21 16 8S14.21 4 12 4 8 5.79 8 8 9.79 12 12 12ZM12 14C9.33 14 4 15.34 4 18V20H20V18C20 15.34 14.67 14 12 14Z"/>
            </svg>
          </div>
          
          <!-- Room Name and Unread Badge -->
          <div class="flex-1 min-w-0 flex items-center justify-between">
            <div class="font-medium truncate">
              {{ room.displayName }}
            </div>
            <div 
              v-if="room.unreadCount > 0" 
              class="flex-shrink-0 ml-2 bg-red-500 text-white text-xs rounded-full min-w-[1.5rem] h-6 flex items-center justify-center px-1"
            >
              {{ room.unreadCount > 99 ? '99+' : room.unreadCount }}
            </div>
          </div>
        </div>
      </button>

      <!-- Leave Room Button -->
      <button
        @click.stop="emit('leave', room)"
        class="flex-shrink-0 m-1 rounded opacity-0 group-hover:opacity-100 transition-all duration-200"
        :class="isActive 
          ? 'text-white' 
          : 'text-gray-400 hover:text-white'
        "
        title="Leave Room"
      >
        <svg class="size-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  room: Room
  isActive: boolean
}>()

const emit = defineEmits<{
  select: [id: string]
  leave: [room: Room]
}>()
</script>