<template>
  <span 
    class="px-2 py-0.5 text-white text-xs rounded-md font-medium"
    :class="{
      'opacity-50': !active,
      'bg-purple-600': mod === 'DT' || mod === 'NC',
      'bg-yellow-600': mod === 'HD',
      'bg-red-600': mod === 'HR',
      'bg-green-600': mod === 'EZ',
      'bg-gray-900': mod === 'FL',
      'bg-orange-600': mod === 'FM',
      'bg-gray-600': mod === 'NF',
      'bg-blue-600': mod === 'HT',
    }"
  >
    {{ mod }}
  </span>
</template>

<script setup lang="ts">
const { mod, active = true } = defineProps<{
  mod: string
  active?: boolean
}>()
</script>