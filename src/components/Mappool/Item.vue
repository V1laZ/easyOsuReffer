<template>
  <button
    type="button"
    class="block w-full rounded-lg border border-slate-800 bg-slate-800/40 p-4 text-left transition-colors hover:border-slate-700 hover:bg-slate-800/80"
    @click="emit('select', mappool)"
  >
    <h3 class="font-medium text-slate-100">
      {{ mappool.name }}
    </h3>
    <p
      v-if="mappool.description"
      class="mt-1 text-sm text-slate-400"
    >
      {{ mappool.description }}
    </p>
    <div class="mt-2 flex items-center justify-between text-xs text-slate-500">
      <span>{{ formattedDate }}</span>
    </div>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Mappool } from '@/types'

const props = defineProps<{
  mappool: Mappool
}>()

const emit = defineEmits<{
  select: [mappool: Mappool]
}>()

const formattedDate = computed(() => {
  return new Date(props.mappool.created_at).toLocaleDateString()
})
</script>
