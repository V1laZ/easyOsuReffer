<template>
  <div
    class="flex items-center mx-1 p-2 px-3 rounded-lg border transition-colors"
    :class="[slotClasses, highlight ? 'ring-2 ring-pink-400' : '']"
    @dragenter="onDragEnter"
    @dragleave="onDragLeave"
    @dragend="highlight = false"
    @dragover.prevent="onDragOver"
    @drop.prevent="onDropEvent"
  >
    <div class="flex-1 min-w-0">
      <div class="flex items-center justify-between">
        <!-- Player -->
        <div
          v-if="slotInfo.player"
          class="flex items-center space-x-2 flex-1 min-w-0"
          draggable="true"
          @dragstart="onDragStart($event, slotInfo.player?.username || 'Unknown')"
        >
          <button
            v-if="slotInfo.player.isHost"
            class="cursor-pointer"
            @click="emit('host', null)"
          >
            <svg
              class="size-6 text-yellow-400"
              xmlns="http://www.w3.org/2000/svg"
              width="32"
              height="32"
              viewBox="0 0 24 24"
            ><!-- Icon from Material Symbols by Google - https://github.com/google/material-design-icons/blob/master/LICENSE --><path
              fill="currentColor"
              d="M5 20v-2h14v2zm0-3.5L3.725 8.475q-.05 0-.113.013T3.5 8.5q-.625 0-1.062-.438T2 7t.438-1.062T3.5 5.5t1.063.438T5 7q0 .175-.038.325t-.087.275L8 9l3.125-4.275q-.275-.2-.45-.525t-.175-.7q0-.625.438-1.063T12 2t1.063.438T13.5 3.5q0 .375-.175.7t-.45.525L16 9l3.125-1.4q-.05-.125-.088-.275T19 7q0-.625.438-1.063T20.5 5.5t1.063.438T22 7t-.437 1.063T20.5 8.5q-.05 0-.112-.012t-.113-.013L19 16.5z"
            /></svg>
          </button>
          <button
            v-else
            class="cursor-pointer"
            @click="emit('host', slotInfo.player.username)"
          >
            <svg
              class="size-6 text-gray-400/80"
              xmlns="http://www.w3.org/2000/svg"
              width="32"
              height="32"
              viewBox="0 0 24 24"
            ><!-- Icon from Material Symbols by Google - https://github.com/google/material-design-icons/blob/master/LICENSE --><path
              fill="currentColor"
              d="M5 20v-2h14v2zm0-3.5L3.725 8.475q-.05 0-.113.013T3.5 8.5q-.625 0-1.062-.438T2 7t.438-1.062T3.5 5.5t1.063.438T5 7q0 .175-.038.325t-.087.275L8 9l3.125-4.275q-.275-.2-.45-.525t-.175-.7q0-.625.438-1.063T12 2t1.063.438T13.5 3.5q0 .375-.175.7t-.45.525L16 9l3.125-1.4q-.05-.125-.088-.275T19 7q0-.625.438-1.063T20.5 5.5t1.063.438T22 7t-.437 1.063T20.5 8.5q-.05 0-.112-.012t-.113-.013L19 16.5zm1.7-2h10.6l.65-4.175l-2.625 1.15L12 6.9l-3.325 4.575l-2.625-1.15zm5.3 0"
            /></svg>
          </button>
          <span class="font-medium text-white truncate flex-1 min-w-0">{{ slotInfo.player.username }}</span>
        </div>

        <div
          v-else
          class="text-gray-500 italic flex-1 min-w-0"
        >
          Empty
        </div>

        <!-- Team -->
        <button
          v-if="slotInfo.player && slotInfo.player.team"
          class="size-4 rounded border-2 flex-shrink-0 cursor-pointer border-gray-600 ml-2"
          :class="teamColor"
          :title="`Team ${slotInfo.player.team}`"
          @click.stop="emit('teamChange', {
            playerName: slotInfo.player.username,
            team: slotInfo.player.team === 'red' ? 'blue' : 'red'
          })"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import type { PlayerSlot, PlayerTeamChangeEvent } from '@/types'

const { slotInfo } = defineProps<{
  slotInfo: PlayerSlot
}>()
const emit = defineEmits<{
  playerMove: [name: string]
  teamChange: [event: PlayerTeamChangeEvent]
  host: [host: string | null]
}>()

const highlight = ref<boolean>(false)
let dragCounter = 0

const slotClasses = computed(() => {
  if (slotInfo.player) {
    if (slotInfo.player.isReady) {
      return 'bg-green-900/30 border-green-600'
    }
    else if (slotInfo.player.isPlaying) {
      return 'bg-blue-900/30 border-blue-600'
    }
    return 'bg-gray-900/30 border-gray-500'
  }
  return 'bg-gray-800 border-gray-600'
})

const onDragStart = (e: DragEvent, playerName: string) => {
  if (!e.dataTransfer) return

  e.dataTransfer.effectAllowed = 'move'
  e.dataTransfer.dropEffect = 'move'

  const original = e.target as HTMLElement
  try {
    const clone = original.cloneNode(true) as HTMLElement
    clone.style.position = 'absolute'
    clone.style.top = '-9999px'
    clone.style.left = '-9999px'
    clone.style.width = `${original.offsetWidth}px`
    clone.style.height = `${original.offsetHeight}px`
    clone.style.boxSizing = 'border-box'
    clone.style.pointerEvents = 'none'
    clone.style.opacity = '1'

    document.body.appendChild(clone)
    e.dataTransfer.setDragImage(clone, clone.offsetWidth / 2, clone.offsetHeight / 2)

    setTimeout(() => document.body.removeChild(clone), 0)
  }
  catch {}

  e.dataTransfer.setData('text/plain', playerName)
}

const onDragOver = (e: DragEvent) => {
  if (!slotInfo.player) {
    e.dataTransfer!.dropEffect = 'move'
  }
  else {
    e.dataTransfer!.dropEffect = 'none'
  }
}

const onDropEvent = (e: DragEvent) => {
  highlight.value = false
  dragCounter = 0

  const playerName = e.dataTransfer?.getData('text/plain')
  if (!playerName || slotInfo.player) return
  emit('playerMove', playerName)
}

const onDragEnter = () => {
  if (slotInfo.player) return
  dragCounter++
  highlight.value = true
}

const onDragLeave = () => {
  dragCounter--
  if (dragCounter <= 0) {
    highlight.value = false
    dragCounter = 0
  }
}

const teamColor = computed(() => {
  if (!slotInfo.player) return 'bg-gray-600'
  return slotInfo.player.team === 'red' ? 'bg-red-500' : 'bg-blue-500'
})
</script>
