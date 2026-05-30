<template>
  <component
    :is="editable ? 'div' : 'button'"
    :type="editable ? undefined : 'button'"
    class="group flex w-full items-stretch overflow-hidden rounded-lg border border-slate-800 bg-slate-800/40 text-left transition-colors hover:border-slate-700 hover:bg-slate-800/80"
    :class="editable ? 'cursor-pointer' : ''"
    @click="editable ? emit('edit') : emit('select', beatmap)"
  >
    <span
      v-if="editable"
      data-drag-handle
      class="flex w-8 shrink-0 cursor-grab touch-none items-center justify-center text-slate-600 transition-colors hover:text-slate-300 active:cursor-grabbing"
      title="Drag to reorder"
      @click.stop
    >
      <Icon
        name="drag"
        size="sm"
      />
    </span>

    <div class="min-w-0 flex-1 py-3 pl-2 pr-3">
      <div class="mb-2 flex items-center gap-2">
        <Badge
          v-if="beatmap.category"
          :tone="categoryTone"
        >
          {{ beatmap.category }}
        </Badge>
        <Badge
          v-if="beatmap.mod_combination"
          tone="accent"
        >
          +{{ beatmap.mod_combination }}
        </Badge>
      </div>

      <h4 class="font-semibold leading-tight text-slate-100 transition-colors group-hover:text-pink-200">
        {{ beatmap.artist }} - {{ beatmap.title }}
      </h4>
      <p class="mt-0.5 text-sm text-slate-300">
        <span>[{{ beatmap.difficulty }}]</span>
        <span class="text-slate-400"> by {{ beatmap.mapper }}</span>
      </p>
      <p class="mt-1 font-mono text-xs text-slate-500">
        ID: {{ beatmap.beatmap_id }}
      </p>
    </div>

    <div
      v-if="canRemove"
      class="flex shrink-0 items-center pr-3"
    >
      <IconBtn
        icon="trash"
        variant="danger"
        size="sm"
        title="Remove from pool"
        :class="editable ? '' : 'md:opacity-0 group-hover:opacity-100'"
        @click.stop="emit('remove')"
      />
    </div>
  </component>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Badge from '@/components/UI/Badge.vue'
import Icon from '@/components/UI/Icon.vue'
import IconBtn from '@/components/UI/IconBtn.vue'
import type { BeatmapEntry } from '@/types'

const { beatmap, canRemove = true, editable = false } = defineProps<{
  beatmap: BeatmapEntry
  canRemove?: boolean
  editable?: boolean
}>()

const emit = defineEmits<{
  remove: []
  select: [beatmap: BeatmapEntry]
  edit: []
}>()

const categoryTone = computed(() => {
  const cat = beatmap.category?.slice(0, 2) || ''
  switch (cat) {
    case 'NM': return 'neutral' as const
    case 'HD': return 'warning' as const
    case 'HR': return 'danger' as const
    case 'DT': return 'info' as const
    case 'FL': return 'neutral' as const
    case 'FM': return 'success' as const
    case 'TB': return 'accent' as const
    default: return 'neutral' as const
  }
})
</script>
