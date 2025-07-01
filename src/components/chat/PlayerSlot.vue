<template>
  <div
    class="flex items-center p-2 rounded-lg border transition-colors"
    :class="slotClasses"
  >
    <!-- Slot Lock Status Icon -->
    <div class="flex-shrink-0 mr-3">
      <svg v-if="slotInfo.isLocked" class="w-5 h-5 text-gray-500" fill="currentColor" viewBox="0 0 24 24">
        <path d="M18,8A2,2 0 0,1 20,10V20A2,2 0 0,1 18,22H6A2,2 0 0,1 4,20V10A2,2 0 0,1 6,8H7V6A5,5 0 0,1 12,1A5,5 0 0,1 17,6V8H18M12,3A3,3 0 0,0 9,6V8H15V6A3,3 0 0,0 12,3Z"/>
      </svg>
      <svg v-else class="w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 24 24">
        <path d="M18,8A2,2 0 0,1 20,10V20A2,2 0 0,1 18,22H6A2,2 0 0,1 4,20V10C4,8.89 4.9,8 6,8H7V6A5,5 0 0,1 12,1A5,5 0 0,1 17,6V8H18M12,3A3,3 0 0,0 9,6V8H15V6A3,3 0 0,0 12,3M6,10V20H18V10H6Z"/>
      </svg>
    </div>

    <!-- Slot Content -->
    <div class="flex-1 min-w-0">
      <div class="flex items-center justify-between">
        <!-- Player -->
        <div class="flex-1 min-w-0">
          <div v-if="slotInfo.player" class="flex items-center space-x-2">
            <span class="font-medium text-white truncate">{{ slotInfo.player.username }}</span>
            <div v-if="slotInfo.player.isHost" class="flex-shrink-0">
              <span class="text-xs bg-yellow-600 text-yellow-100 px-2 py-1 rounded">HOST</span>
            </div>
          </div>
          <div v-else class="text-gray-500 italic">Empty</div>
        </div>

        <!-- Team -->
        <div 
          v-if="slotInfo.player && slotInfo.player.team"
          class="size-4 rounded border-2 flex-shrink-0 border-gray-600"
          :class="teamColor"
          :title="`Team ${slotInfo.player.team}`"
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  slotInfo: PlayerSlot
}>()

const slotClasses = computed(() => {
    const baseClasses = []
  
  if (props.slotInfo.isLocked) {
    baseClasses.push('bg-gray-700 border-gray-600')
  } else if (props.slotInfo.player) {
    // Player is in slot
    if (props.slotInfo.player.isReady) {
      baseClasses.push('bg-green-900/30 border-green-600')
    } else if (props.slotInfo.player.isPlaying) {
      baseClasses.push('bg-blue-900/30 border-blue-600')
    } else {
      baseClasses.push('bg-gray-900/30 border-gray-500')
    }
  } else {
    // Empty slot
    baseClasses.push('bg-gray-800 border-gray-600 hover:bg-gray-750')
  }
  
  return baseClasses.join(' ')
})

const teamColor = computed(() => {
  if (!props.slotInfo.player) return 'bg-gray-600'
  return props.slotInfo.player.team === 'red' ? 'bg-red-500' : 'bg-blue-500'
})
</script>