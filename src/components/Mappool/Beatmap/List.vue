<template>
  <div class="flex-1 mt-px overflow-y-auto">
    <div v-if="beatmaps.length === 0" class="flex flex-col items-center justify-center h-full text-gray-500 p-8">
      <svg class="w-16 h-16 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
      </svg>
      <p>No beatmaps in this pool</p>
      <p class="text-sm text-gray-600 mt-1">Add some maps to get started</p>
    </div>

    <div v-else class="space-y-3 md:space-y-1 p-2">
      <Item 
        v-for="beatmap in groupedBeatmaps"
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
import Item from './Item.vue';
import { dbService } from '../../../services/database';

const { beatmaps = [], canRemove = true} = defineProps<{
  beatmaps: BeatmapEntry[]
  canRemove?: boolean
}>()

const emit = defineEmits<{
  remove: []
  select: [beatmap: BeatmapEntry]
}>()

const categoryOrder = ['NM', 'HD', 'HR', 'DT', 'FM', 'TB'];

const groupedBeatmaps = computed(() => {
  return [...beatmaps].sort((a, b) => {
    const getPrefix = (cat?: string) => (cat ? cat.slice(0, 2) : '');
    const aPrefix = getPrefix(a.category);
    const bPrefix = getPrefix(b.category);

    const aIndex = categoryOrder.indexOf(aPrefix) !== -1 ? categoryOrder.indexOf(aPrefix) : categoryOrder.length - 1;
    const bIndex = categoryOrder.indexOf(bPrefix) !== -1 ? categoryOrder.indexOf(bPrefix) : categoryOrder.length - 1;

    if (aIndex !== bIndex) {
      return aIndex - bIndex;
    }
    return (a.category || '').localeCompare(b.category || '');
  });
});

const removeBeatmap = async (beatmapId: number) => {
  if (!confirm('Are you sure you want to remove this beatmap?')) return
  
  try {
    await dbService.deleteBeatmapFromPool(beatmapId)
    emit('remove')
  } catch (error) {
    console.error('Failed to remove beatmap:', error)
    alert('Failed to remove beatmap')
  }
}
</script>