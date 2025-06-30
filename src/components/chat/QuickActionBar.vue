<template>
  <div class="bg-gray-750 border-b border-gray-600 p-3">
    <div class="flex items-center justify-between">
      <!-- Match Status -->
      <div class="flex items-center space-x-3">
        <div class="flex items-center space-x-2">
          <div 
            class="w-3 h-3 rounded-full"
            :class="matchStatus === 'active' ? 'bg-green-500' : matchStatus === 'starting' ? 'bg-yellow-500' : 'bg-gray-500'"
          ></div>
          <span class="text-sm font-medium text-white">
            {{ getMatchStatusText() }}
          </span>
        </div>
        
        <!-- Current Map Info (if any) -->
        <div v-if="currentMap" class="hidden sm:flex items-center space-x-2 text-sm text-gray-300">
          <span>•</span>
          <span class="truncate max-w-48">{{ currentMap.title }} [{{ currentMap.difficulty }}]</span>
        </div>
      </div>

      <!-- Quick Actions -->
      <div class="flex items-center space-x-2">
        <!-- Mobile Actions (Condensed) -->
        <div class="flex sm:hidden items-center space-x-1">
          <button
            @click="handleQuickAction('start')"
            :disabled="matchStatus === 'active'"
            class="px-3 py-1.5 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white text-sm rounded-lg transition-colors"
          >
            Start
          </button>
          
          <button
            class="p-1.5 hover:bg-gray-600 rounded-lg transition-colors"
            title="More actions"
          >
            <svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"/>
            </svg>
          </button>
        </div>

        <!-- Desktop Actions (Full) -->
        <div class="hidden sm:flex items-center space-x-2">
          <!-- Match Control Buttons -->
          <div class="flex items-center space-x-1">
            <button
              @click="handleQuickAction('start')"
              :disabled="matchStatus === 'active'"
              class="flex items-center space-x-1 px-3 py-1.5 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white text-sm rounded-lg transition-colors"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1m4 0h1m-6 4h1m4 0h1m6-7V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2V9l-6-6z" />
              </svg>
              <span>Start Match</span>
            </button>

            <button
              @click="handleQuickAction('abort')"
              :disabled="matchStatus !== 'active'"
              class="flex items-center space-x-1 px-3 py-1.5 bg-red-600 hover:bg-red-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white text-sm rounded-lg transition-colors"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
              <span>Abort</span>
            </button>
          </div>

          <!-- Map Selection -->
          <div class="flex items-center space-x-1">
            <button
              @click="handleQuickAction('select-map')"
              class="flex items-center space-x-1 px-3 py-1.5 bg-purple-600 hover:bg-purple-700 text-white text-sm rounded-lg transition-colors"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
              </svg>
              <span>Select Map</span>
            </button>

            <button
              @click="handleQuickAction('random-map')"
              class="flex items-center space-x-1 px-3 py-1.5 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded-lg transition-colors"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
              <span>Random</span>
            </button>
          </div>

          <!-- Additional Controls -->
          <div class="flex items-center space-x-1 pl-2 border-l border-gray-600">
            <button
              @click="handleQuickAction('settings')"
              class="p-1.5 hover:bg-gray-600 rounded-lg transition-colors"
              title="Match Settings"
            >
              <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
            </button>

            <button
              @click="handleQuickAction('invite')"
              class="p-1.5 hover:bg-gray-600 rounded-lg transition-colors"
              title="Invite Players"
            >
              <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Mobile Current Map Info -->
    <div v-if="currentMap" class="sm:hidden mt-2 pt-2 border-t border-gray-600">
      <div class="text-sm text-gray-300">
        <span class="font-medium">{{ currentMap.title }}</span>
        <span class="text-gray-400"> [{{ currentMap.difficulty }}]</span>
        <span class="text-gray-400 ml-2">by {{ currentMap.mapper }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  channel: string
}

interface CurrentMap {
  title: string
  difficulty: string
  mapper: string
  artist: string
}

defineProps<Props>()

// Mock data - in real implementation this would come from match state
const matchStatus = ref<'idle' | 'starting' | 'active'>('idle')
const currentMap = ref<CurrentMap | null>(null)

// Example current map
// currentMap.value = {
//   title: 'Blue Zenith',
//   difficulty: 'FOUR DIMENSIONS',
//   mapper: 'Asphyxia',
//   artist: 'xi'
// }

const getMatchStatusText = (): string => {
  switch (matchStatus.value) {
    case 'active':
      return 'Match in progress'
    case 'starting':
      return 'Starting match...'
    default:
      return 'Match ready'
  }
}

const handleQuickAction = (action: string) => {
  console.log('Quick action:', action)
  
  switch (action) {
    case 'start':
      // TODO: Implement match start
      matchStatus.value = 'starting'
      setTimeout(() => {
        matchStatus.value = 'active'
      }, 2000)
      break
    case 'abort':
      // TODO: Implement match abort
      matchStatus.value = 'idle'
      break
    case 'select-map':
      // TODO: Open map selection modal
      break
    case 'random-map':
      // TODO: Select random map from mappool
      break
    case 'settings':
      // TODO: Open match settings modal
      break
    case 'invite':
      // TODO: Open player invite modal
      break
  }
}
</script>

<style scoped>
.bg-gray-750 {
  background-color: #374151;
}
</style>
