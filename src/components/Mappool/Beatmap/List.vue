<template>
  <div class="flex flex-1 flex-col overflow-y-auto">
    <div
      v-if="beatmaps.length === 0"
      class="flex h-full flex-col items-center justify-center p-8 text-slate-500"
    >
      <Icon
        name="musicCollection"
        size="xl"
        class="mb-3 text-slate-600"
      />
      <p class="text-sm">
        No beatmaps in this pool
      </p>
      <p class="mt-1 text-xs text-slate-600">
        Add some maps to get started
      </p>
    </div>

    <div
      v-else
      class="space-y-2 p-3"
    >
      <Item
        v-for="beatmap in groupedBeatmaps"
        :key="beatmap.id"
        :beatmap="beatmap"
        :can-remove="canRemove"
        @remove="removeBeatmap(beatmap.id)"
        @select="emit('select', beatmap)"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import Item from './Item.vue'
import Icon from '@/components/UI/Icon.vue'
import { dbService } from '@/services/database'
import { confirm } from '@/composables/useConfirm'
import type { BeatmapEntry } from '@/types'

const { beatmaps = [], canRemove = true } = defineProps<{
  beatmaps?: BeatmapEntry[]
  canRemove?: boolean
}>()

const emit = defineEmits<{
  remove: []
  select: [beatmap: BeatmapEntry]
}>()

const categoryOrder = ['NM', 'HD', 'HR', 'DT', 'FM', 'TB']

const groupedBeatmaps = computed(() => {
  return [...beatmaps].sort((a, b) => {
    const getPrefix = (cat?: string) => (cat ? cat.slice(0, 2) : '')
    const aPrefix = getPrefix(a.category)
    const bPrefix = getPrefix(b.category)

    const aIndex = categoryOrder.indexOf(aPrefix)
    const bIndex = categoryOrder.indexOf(bPrefix)

    const normAIndex = aIndex === -1 ? categoryOrder.length - 2 : aIndex
    const normBIndex = bIndex === -1 ? categoryOrder.length - 2 : bIndex

    if (normAIndex !== normBIndex) {
      return normAIndex - normBIndex
    }
    return (a.category || '').localeCompare(b.category || '')
  })
})

const removeBeatmap = async (beatmapId: number) => {
  const ok = await confirm({
    title: 'Remove beatmap?',
    message: 'The beatmap will be removed from this mappool.',
    confirmText: 'Remove',
    tone: 'danger',
  })
  if (!ok) return

  try {
    await dbService.deleteBeatmapFromPool(beatmapId)
    emit('remove')
  }
  catch (error) {
    console.error('Failed to remove beatmap:', error)
    alert('Failed to remove beatmap')
  }
}
</script>
