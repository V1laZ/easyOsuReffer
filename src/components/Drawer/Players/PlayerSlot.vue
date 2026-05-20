<template>
  <div
    :class="[
      'flex items-center rounded-lg px-3 py-2 ring-1 ring-inset transition-colors',
      slotClasses,
      highlight ? 'ring-pink-400/60' : '',
    ]"
    @dragenter="onDragEnter"
    @dragleave="onDragLeave"
    @dragend="highlight = false"
    @dragover.prevent="onDragOver"
    @drop.prevent="onDropEvent"
  >
    <div class="flex min-w-0 flex-1 items-center justify-between">
      <div
        v-if="slotInfo.player"
        class="flex min-w-0 flex-1 items-center gap-2"
        draggable="true"
        @dragstart="onDragStart($event, slotInfo.player?.username || 'Unknown')"
      >
        <button
          class="flex-shrink-0 cursor-pointer transition-colors"
          :class="slotInfo.player.isHost ? 'text-amber-300 hover:text-amber-200' : 'text-slate-500 hover:text-slate-300'"
          :title="slotInfo.player.isHost ? 'Clear host' : 'Make host'"
          @click="emit('host', slotInfo.player.isHost ? null : slotInfo.player.username)"
        >
          <Icon
            :name="slotInfo.player.isHost ? 'crown' : 'crownOutline'"
            size="md"
          />
        </button>
        <span class="min-w-0 flex-1 truncate text-sm font-medium text-slate-100">
          {{ slotInfo.player.username }}
        </span>
      </div>

      <div
        v-else
        class="min-w-0 flex-1 text-sm italic text-slate-500"
      >
        Empty
      </div>

      <button
        v-if="slotInfo.player && slotInfo.player.team"
        class="ml-2 size-4 flex-shrink-0 cursor-pointer rounded ring-2 ring-inset"
        :class="teamColor"
        :title="`Team ${slotInfo.player.team}`"
        @click.stop="emit('teamChange', {
          playerName: slotInfo.player.username,
          team: slotInfo.player.team === 'red' ? 'blue' : 'red'
        })"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import Icon from '@/components/UI/Icon.vue'
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
      return 'bg-emerald-500/10 ring-emerald-400/30'
    }
    if (slotInfo.player.isPlaying) {
      return 'bg-sky-500/10 ring-sky-400/30'
    }
    return 'bg-slate-800/80 ring-slate-700'
  }
  return 'bg-slate-800/40 ring-slate-800'
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
  catch {
    // ignore
  }

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
  if (!slotInfo.player) return 'bg-slate-600 ring-slate-500'
  return slotInfo.player.team === 'red'
    ? 'bg-rose-400 ring-rose-300/50'
    : 'bg-sky-400 ring-sky-300/50'
})
</script>
