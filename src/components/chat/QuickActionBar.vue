<template>
  <div class="bg-[#374151] border-b border-gray-600 p-3">
    <div class="flex items-center justify-between">
      <!-- Match Status -->
      <div class="flex items-center space-x-3">
        <div class="flex items-center space-x-2">
          <div
            class="w-3 h-3 rounded-full"
            :class="{
              'bg-green-500': room.lobbyState.matchStatus === 'ready',
              'bg-yellow-500': room.lobbyState.matchStatus === 'active',
              'bg-yellow-500/50': room.lobbyState.matchStatus === 'starting',
              'bg-gray-500': !room.lobbyState || room.lobbyState.matchStatus === 'idle'
            }"
          />
          <span class="text-sm font-medium text-white">
            {{ matchStatusText }}
          </span>
          <span
            v-if="formattedTime"
            class="text-sm font-mono text-yellow-400"
          >
            {{ formattedTime }}
          </span>
        </div>
      </div>

      <!-- Quick Actions -->
      <div class="flex items-center space-x-2">
        <div class="flex items-center space-x-1">
          <template v-if="room.lobbyState.matchStatus === 'active'">
            <button
              class="px-3 py-1.5 bg-red-600 hover:bg-red-700 text-white text-sm rounded-lg transition-colors"
              @click="handleAbort"
            >
              Abort
            </button>
          </template>
          <template v-else>
            <button
              :disabled="!currentMap"
              class="px-3 py-1.5 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white text-sm rounded-lg transition-colors"
              @click="emit('sendMessage', '!mp start 10')"
            >
              Start
            </button>
            <button
              class="px-3 py-1.5 bg-purple-600 hover:bg-purple-700 text-white text-sm rounded-lg transition-colors"
              @click="emit('openSelectMap')"
            >
              Change Map
            </button>
          </template>
        </div>
      </div>
    </div>

    <!-- Current Map Info -->
    <div
      v-if="currentMap"
      class="mt-2 pt-2 border-t border-gray-600"
    >
      <div class="text-sm text-gray-300">
        <span class="font-medium">{{ currentMap.title }}</span>
        <span
          v-if="currentMap.difficulty"
          class="text-gray-400"
        > [{{ currentMap.difficulty }}]</span>
      </div>
    </div>

    <!-- Mods Info -->
    <div
      v-if="currentMap"
      class="mt-2"
    >
      <div class="flex items-center space-x-2">
        <span class="text-sm text-gray-400">Mods:</span>
        <div class="flex items-center space-x-1">
          <Mod
            v-for="mod in room.lobbyState.selectedMods"
            :key="mod"
            :mod="mod"
          />
          <span
            v-if="room.lobbyState.freemod"
            class="px-2 py-0.5 bg-orange-600 text-white text-xs rounded-md font-medium"
          >
            Freemod
          </span>
          <span
            v-else-if="room.lobbyState.selectedMods.length === 0"
            class="px-2 py-0.5 bg-gray-600 text-white text-xs rounded-md font-medium"
          >
            NoMod
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Mod from '../Mod.vue'
import { useMatchCountdown } from '@/composables/useMatchCountdown'
import type { MultiplayerRoom } from '@/types'

const props = defineProps<{
  room: MultiplayerRoom
}>()

const emit = defineEmits<{
  openSelectMap: []
  sendMessage: [message: string]
}>()

const currentMap = computed(() => props.room.lobbyState.currentMap || null)
const lobbyState = computed(() => props.room.lobbyState)
const roomId = computed(() => props.room.id)

const { formattedTime } = useMatchCountdown(lobbyState, roomId)

function handleAbort() {
  if (confirm('Are you sure you want to abort the match?')) {
    emit('sendMessage', '!mp abort')
  }
}

const matchStatusText = computed(() => {
  switch (props.room.lobbyState.matchStatus) {
    case 'active': return 'In progress'
    case 'starting': return 'Starting...'
    case 'ready': return 'Ready'
    default: return 'Idle'
  }
})
</script>
