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
            v-if="formattedMatchTime"
            class="text-sm font-mono text-yellow-400"
          >
            {{ formattedMatchTime }}
          </span>
          <span
            v-if="timerIsActive"
            class="text-sm font-mono text-pink-400"
          >
            {{ formattedTimerTime }}
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
            <div class="relative">
              <button
                :class="timerIsActive
                  ? 'bg-red-600 hover:bg-red-700 hover:border-red-700 border-red-600 text-white'
                  : 'bg-gray-500/20 hover:bg-gray-500/30 text-white border-gray-500/40'"
                class="p-1.5 border rounded-lg transition-colors"
                :title="timerIsActive ? 'Abort countdown' : 'Start countdown timer'"
                @click="handleTimerButtonClick"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  class="w-4 h-4"
                >
                  <circle
                    cx="12"
                    cy="13"
                    r="8"
                  />
                  <polyline points="12 9 12 13 14.5 15.5" />
                  <line
                    x1="10"
                    y1="2"
                    x2="14"
                    y2="2"
                  />
                  <line
                    x1="12"
                    y1="2"
                    x2="12"
                    y2="5"
                  />
                </svg>
              </button>

              <!-- Timer popup -->
              <div
                v-if="showTimerPopup"
                class="absolute right-0 top-full mt-2 z-50 w-56 bg-gray-800 border border-gray-500/30 rounded-xl shadow-xl p-4"
                @click.self="showTimerPopup = false"
              >
                <p class="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-3">
                  Countdown Timer
                </p>
                <div class="flex items-center gap-2 mb-4">
                  <div class="flex-1">
                    <label class="block text-xs text-gray-400 mb-1">Minutes</label>
                    <input
                      v-model.number="timerMinutes"
                      type="number"
                      min="0"
                      max="59"
                      class="w-full bg-gray-700/50 border border-gray-600 text-white text-sm rounded-lg px-2 py-1.5 focus:outline-none focus:border-pink-500 text-center"
                    >
                  </div>
                  <span class="text-gray-400 text-lg font-bold mt-4">:</span>
                  <div class="flex-1">
                    <label class="block text-xs text-gray-400 mb-1">Seconds</label>
                    <input
                      v-model.number="timerSeconds"
                      type="number"
                      min="0"
                      max="59"
                      class="w-full bg-gray-700/50 border border-gray-600 text-white text-sm rounded-lg px-2 py-1.5 focus:outline-none focus:border-pink-500 text-center"
                    >
                  </div>
                </div>
                <button
                  :disabled="timerTotalSeconds <= 0"
                  class="w-full py-1.5 bg-pink-600 hover:bg-pink-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white text-sm font-medium rounded-lg transition-colors"
                  @click="startTimer"
                >
                  Start Timer
                </button>
              </div>
            </div>
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
import { computed, ref } from 'vue'
import Mod from '../Mod.vue'
import { useMatchCountdown } from '@/composables/useMatchCountdown'
import { useTimerCountdown } from '@/composables/useTimerCountdown'
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

const { formattedTime: formattedMatchTime } = useMatchCountdown(lobbyState, roomId)
const { formattedTime: formattedTimerTime, isActive: timerIsActive } = useTimerCountdown(lobbyState)

// Timer popup state
const showTimerPopup = ref(false)
const timerMinutes = ref(0)
const timerSeconds = ref(30)
const timerTotalSeconds = computed(() => timerMinutes.value * 60 + timerSeconds.value)

function handleTimerButtonClick() {
  if (timerIsActive.value) {
    if (confirm('Are you sure you want to abort the countdown?')) {
      emit('sendMessage', '!mp aborttimer')
    }
  }
  else {
    showTimerPopup.value = !showTimerPopup.value
  }
}

function startTimer() {
  if (timerTotalSeconds.value <= 0) return
  emit('sendMessage', `!mp timer ${timerTotalSeconds.value}`)
  showTimerPopup.value = false
}

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
