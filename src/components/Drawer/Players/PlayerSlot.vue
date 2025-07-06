<template>
  <div
    class="flex items-center p-2 px-3 rounded-lg border transition-colors"
    :class="[slotClasses, highlight ? 'ring-2 ring-pink-400' : '']"
    :draggable="!!slotInfo.player"
    @dragstart="onDragStart($event, slotInfo.player?.username || 'Unknown')"
    @dragenter="onDragEnter"
    @dragleave="onDragLeave"
    @dragend="highlight = false"
    @dragover.prevent
    @drop.prevent="onDropEvent"
  >
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
import { computed, ref } from 'vue';

const { slotInfo } = defineProps<{
  slotInfo: PlayerSlot
}>()
const emit = defineEmits<{
  playerMove: [name: string]
}>()

const highlight = ref<boolean>(false)
let dragCounter = 0

const slotClasses = computed(() => {
  if (slotInfo.player) {
    if (slotInfo.player.isReady) {
      return 'bg-green-900/30 border-green-600'
    } else if (slotInfo.player.isPlaying) {
      return 'bg-blue-900/30 border-blue-600'
    } 
    return 'bg-gray-900/30 border-gray-500'
  } 
  return 'bg-gray-800 border-gray-600'
})

const onDragStart = (e: DragEvent, playerName: string) => {
  if (!e.dataTransfer) return

  const original = e.target as HTMLElement
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

  e.dataTransfer.setData('text/plain', playerName)
}

const onDropEvent = (e: DragEvent) => {
  highlight.value = false
  dragCounter = 0

  const playerName = e.dataTransfer?.getData('text/plain')
  if (!playerName || playerName === slotInfo.player?.username) return
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