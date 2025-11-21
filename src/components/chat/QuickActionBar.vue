<template>
  <div class="bg-[#374151] border-b border-gray-600 p-3">
    <div class="flex items-center justify-between">
      <!-- Match Status -->
      <div class="flex items-center space-x-3">
        <div class="flex items-center space-x-2">
          <div
            class="w-3 h-3 rounded-full"
            :class="{
              'bg-green-500': lobbyState.matchStatus === 'ready',
              'bg-yellow-500': lobbyState.matchStatus === 'active',
              'bg-yellow-500/50': lobbyState.matchStatus === 'starting',
              'bg-gray-500': !lobbyState || lobbyState.matchStatus === 'idle'
            }"
          />
          <span class="text-sm font-medium text-white">
            {{ matchStatusText }}
          </span>
        </div>
      </div>

      <!-- Quick Actions -->
      <div class="flex items-center space-x-2">
        <div class="flex items-center space-x-1">
          <button
            :disabled="!lobbyState || lobbyState.matchStatus === 'active' || !currentMap"
            class="px-3 py-1.5 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white text-sm rounded-lg transition-colors"
            @click="handleQuickAction('start')"
          >
            Start
          </button>
          <button
            :disabled="!lobbyState || lobbyState.matchStatus === 'active'"
            class="px-3 py-1.5 bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white text-sm rounded-lg transition-colors"
            @click="emit('openSelectMap')"
          >
            Change Map
          </button>
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
            v-for="mod in lobbyState.selectedMods"
            :key="mod"
            :mod="mod"
          />
          <span
            v-if="lobbyState.freemod"
            class="px-2 py-0.5 bg-orange-600 text-white text-xs rounded-md font-medium"
          >
            Freemod
          </span>
          <span
            v-else-if="lobbyState.selectedMods.length === 0"
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
import { invoke } from '@tauri-apps/api/core'
import Mod from '../Mod.vue'

const props = defineProps<{
  lobbyState: LobbyState
}>()

const emit = defineEmits<{
  openSelectMap: []
}>()

const currentMap = computed(() => props.lobbyState.currentMap || null)

const matchStatusText = computed(() => {
  if (!props.lobbyState) return 'Idle'

  switch (props.lobbyState.matchStatus) {
    case 'active':
      return 'In progress'
    case 'starting':
      return 'Starting...'
    case 'idle':
      return 'Idle'
    case 'ready':
      return 'Ready'
    default:
      return 'Idle'
  }
})

const handleQuickAction = (action: string) => {
  switch (action) {
    case 'start':
      invoke('send_message_to_room', {
        roomId: props.lobbyState.channel,
        message: '!mp start 10',
      })
      break
    case 'abort':
      invoke('send_message_to_room', {
        roomId: props.lobbyState.channel,
        message: '!mp abort',
      })
      break
  }
}
</script>
