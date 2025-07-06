<template>
  <div 
    class="fixed max-w-full inset-y-0 right-0 z-40 min-w-80 bg-gray-800 border-l border-gray-700 transform transition-transform duration-300 ease-in-out lg:relative lg:translate-x-0 flex flex-col h-full"
    :class="isOpen ? 'translate-x-0' : 'translate-x-full'"
  >
    <!-- Header -->
    <div class="flex items-start justify-between p-4 pb-3 border-b border-gray-700">
      <div>
        <h2 class="text-lg font-semibold text-white">Player Slots</h2>
        <div class="text-xs text-gray-400">
          <span>Ready: </span>
          <span>{{ readyPlayers }}/{{ occupiedSlots }}</span>
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
    <div class="flex-1 flex flex-col p-3 min-h-0">
      <div class="space-y-2 flex-1 overflow-y-auto min-h-0">
        <PlayerSlot
          v-for="(slot, idx) in lobbyState.slots"
          :key="slot.id"
          :slot-info="slot"
          @player-move="emit('move', { playerName: $event, to: idx + 1})"
          @team-change="emit('teamChange', $event)"
          @host="emit('host', $event)"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import PlayerSlot from './PlayerSlot.vue'

const props = defineProps<{
  isOpen: boolean
  lobbyState: LobbyState
}>()

const emit = defineEmits<{
  close: []
  move: [move: PlayerMoveEvent]
  teamChange: [event: PlayerTeamChangeEvent]
  host: [host: string | null]
}>()

const occupiedSlots = computed(() => 
  props.lobbyState.slots.filter(slot => slot.player !== null).length || 0
)

const readyPlayers = computed(() => 
  props.lobbyState.slots.filter(slot => slot.player?.isReady).length || 0
)
</script>
