<template>
  <div
    :class="[
      'rounded-lg border border-slate-800 bg-slate-900',
      paddingClass,
    ]"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const { padding = 'md' } = defineProps<{
  padding?: 'none' | 'sm' | 'md' | 'lg'
}>()

const paddingClass = computed(() => ({
  none: '',
  sm: 'p-3',
  md: 'p-4',
  lg: 'p-6',
}[padding]))
</script>
