<template>
  <div 
    class="fixed overflow-scroll inset-y-0 right-0 z-40 w-80 bg-gray-800 border-l border-gray-700 transform transition-transform duration-300 ease-in-out lg:relative lg:translate-x-0"
    :class="isOpen ? 'translate-x-0' : 'translate-x-full'"
  >
    <!-- Header -->
    <div class="flex items-start justify-between p-4 pb-3 border-b border-gray-700">
      <div>
        <h2 class="text-lg font-semibold text-white">Player Slots</h2>
        <!-- Room Info -->
        <div class="text-xs flex items-center gap-2 text-gray-400">
          <div>
            <span>Players: </span>
            <span>{{ occupiedSlots }}/{{ unlockedSlots }}</span>
          </div>
          <div>
            <span>Ready: </span>
            <span>{{ readyPlayers }}/{{ occupiedSlots }}</span>
          </div>
        </div>
      </div>
      <button 
        @click="emit('close')"
        class="lg:hidden p-2 rounded-lg hover:bg-gray-700 transition-colors"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Player Slots -->
    <div class="flex-1 overflow-y-scroll p-3">
      <div class="space-y-2">
        <PlayerSlot
          v-for="slot in slots"
          :key="slot.id"
          :slot-info="slot"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import PlayerSlot from './PlayerSlot.vue'

interface Player {
  username: string
  team: 'red' | 'blue'
  isReady: boolean
  isPlaying: boolean
  isHost: boolean
}

interface Slot {
  id: number
  isLocked: boolean
  player: Player | null
}

defineProps<{
  isOpen: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

// Mock data for demonstration - in real app this would come from props or API
const mockSlots: Slot[] = [
  { id: 1, isLocked: false, player: { username: 'PlayerOne', team: 'red', isReady: true, isPlaying: true, isHost: true } },
  { id: 2, isLocked: false, player: { username: 'CoolGamer123', team: 'red', isReady: false, isPlaying: true, isHost: false } },
  { id: 3, isLocked: false, player: { username: 'osu_pro_2024', team: 'blue', isReady: true, isPlaying: true, isHost: false } },
  { id: 4, isLocked: false, player: null },
  { id: 5, isLocked: false, player: { username: 'MegaMapper', team: 'blue', isReady: true, isPlaying: false, isHost: false } },
  { id: 6, isLocked: false, player: null },
  { id: 7, isLocked: false, player: null },
  { id: 8, isLocked: false, player: { username: 'RhythmMaster', team: 'red', isReady: false, isPlaying: false, isHost: false } },
  { id: 9, isLocked: true, player: null },
  { id: 10, isLocked: true, player: null },
  { id: 11, isLocked: true, player: null },
  { id: 12, isLocked: true, player: null },
  { id: 13, isLocked: true, player: null },
  { id: 14, isLocked: true, player: null },
  { id: 15, isLocked: true, player: null },
  { id: 16, isLocked: true, player: null },
]

const slots = computed(() => mockSlots)

const unlockedSlots = computed(() => 
  slots.value.filter(slot => !slot.isLocked).length
)

const occupiedSlots = computed(() => 
  slots.value.filter(slot => slot.player !== null).length
)

const readyPlayers = computed(() => 
  slots.value.filter(slot => slot.player?.isReady).length
)

</script>
