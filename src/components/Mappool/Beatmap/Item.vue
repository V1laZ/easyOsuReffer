<template>
  <button
    type="button"
    class="group block w-full overflow-hidden rounded-lg border border-slate-800 bg-slate-800/40 text-left transition-colors hover:border-slate-700 hover:bg-slate-800/80"
    @click="emit('select', beatmap)"
  >
    <div class="p-4">
      <div class="flex items-start gap-3">
        <div class="min-w-0 flex-1">
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

        <IconBtn
          v-if="canRemove"
          icon="trash"
          variant="danger"
          size="sm"
          title="Remove from pool"
          class="md:opacity-0 group-hover:opacity-100"
          @click.stop="emit('remove')"
        />
      </div>
    </div>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Badge from '@/components/UI/Badge.vue'
import IconBtn from '@/components/UI/IconBtn.vue'
import type { BeatmapEntry } from '@/types'

const { beatmap, canRemove = true } = defineProps<{
  beatmap: BeatmapEntry
  canRemove?: boolean
}>()

const emit = defineEmits<{
  remove: []
  select: [beatmap: BeatmapEntry]
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
