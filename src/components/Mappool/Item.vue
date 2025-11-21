<template>
  <div
    class="p-4 bg-gray-800 rounded-lg cursor-pointer border border-gray-700 hover:scale-[1.02] duration-300 transition-transform"
    @click="emit('select', mappool)"
  >
    <h3 class="text-white font-medium">
      {{ mappool.name }}
    </h3>
    <p
      v-if="mappool.description"
      class="text-sm text-gray-400 mt-1"
    >
      {{ mappool.description }}
    </p>
    <div class="flex items-center justify-between mt-2">
      <span class="text-xs text-gray-500">{{ formattedDate }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

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
