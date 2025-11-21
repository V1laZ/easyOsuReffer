<template>
  <div class="flex-1 overflow-y-auto">
    <div
      v-if="mappools.length === 0"
      class="flex flex-col items-center justify-center h-full text-gray-500 p-8"
    >
      <svg
        class="w-16 h-16 mb-4"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"
        />
      </svg>
      <p class="text-center">
        No mappools yet
      </p>
      <p class="text-sm text-gray-600 mt-1">
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
        @select="emit('select', mappool)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import Item from './Item.vue'
import type { Mappool } from '@/types'

defineProps<{
  mappools: Mappool[]
}>()

const emit = defineEmits<{
  select: [mappool: Mappool]
}>()
</script>
