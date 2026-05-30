<template>
  <div class="flex-1 overflow-y-auto">
    <div
      v-if="mappools.length === 0"
      class="flex h-full flex-col items-center justify-center p-8 text-slate-500"
    >
      <Icon
        name="musicCollection"
        size="xl"
        class="mb-3 text-slate-600"
      />
      <p class="text-sm">
        No mappools yet
      </p>
      <p class="mt-1 text-xs text-center text-slate-600">
        Create your first mappool to get started
      </p>
    </div>

    <div
      v-else
      class="space-y-2 p-4"
    >
      <Item
        v-for="mappool in mappools"
        :key="mappool.id"
        :mappool="mappool"
        :active="mappool.id === selectedId"
        @select="emit('select', mappool)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import Item from './Item.vue'
import Icon from '@/components/UI/Icon.vue'
import type { Mappool } from '@/types'

defineProps<{
  mappools: Mappool[]
  selectedId?: number
}>()

const emit = defineEmits<{
  select: [mappool: Mappool]
}>()
</script>
