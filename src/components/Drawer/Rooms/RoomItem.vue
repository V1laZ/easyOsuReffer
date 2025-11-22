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
        class="flex-1 flex items-center px-3 py-2 text-left min-w-0"
        @click="emit('select', room.id)"
      >
        <div class="flex items-center space-x-3 flex-1 min-w-0">
          <!-- Room Icon -->
          <div class="flex-shrink-0">
            <svg
              v-if="room.roomType === 'Channel'"
              class="w-5 h-5"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path d="M5.41 21L6.12 17H2.12L2.47 15H6.47L7.53 9H3.53L3.88 7H7.88L8.59 3H10.59L9.88 7H15.88L16.59 3H18.59L17.88 7H21.88L21.53 9H17.53L16.47 15H20.47L20.12 17H16.12L15.41 21H13.41L14.12 17H8.12L7.41 21H5.41ZM9.53 9L8.47 15H14.47L15.53 9H9.53Z" />
            </svg>
            <svg
              v-else-if="room.roomType === 'MultiplayerLobby'"
              class="w-5 h-5"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path d="M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5s-3 1.34-3 3 1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V19h14v-2.5C15 14.17 10.33 13 8 13zm8 0c-.29 0-.62.02-.97.05C15.64 13.36 17 14.28 17 15.5V19h6v-2.5c0-2.33-4.67-3.5-7-3.5z" />
            </svg>
            <svg
              v-else
              class="w-5 h-5"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path d="M12 12C14.21 12 16 10.21 16 8S14.21 4 12 4 8 5.79 8 8 9.79 12 12 12ZM12 14C9.33 14 4 15.34 4 18V20H20V18C20 15.34 14.67 14 12 14Z" />
            </svg>
          </div>

          <!-- Room Name and Unread Badge -->
          <div class="flex-1 min-w-0 flex items-center justify-between">
            <div class="font-medium truncate">
              {{ room.displayName }}
            </div>
          </div>
        </div>
      </button>

      <!-- Leave Room Button -->
      <button
        class="flex-shrink-0 m-1 rounded opacity-100 md:opacity-0 group-hover:opacity-100 transition-all duration-200"
        :class="isActive
          ? 'text-white'
          : 'text-gray-400 hover:text-white'
        "
        title="Leave Room"
        @click.stop="emit('leave', room)"
      >
        <svg
          class="size-6"
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
  </div>
</template>

<script setup lang="ts">
import type { RoomListItem } from '@/types'

defineProps<{
  room: RoomListItem
  isActive: boolean
}>()

const emit = defineEmits<{
  select: [id: string]
  leave: [room: RoomListItem]
}>()
</script>
