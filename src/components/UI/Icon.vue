<template>
  <svg
    :class="sizeClass"
    :viewBox="def.viewBox"
    :fill="def.fill"
    :stroke="def.stroke"
    :stroke-width="def.strokeWidth"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
    v-html="def.body"
  />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ICONS, type IconName } from './icons'

const { name, size = 'md' } = defineProps<{
  name: IconName
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
}>()

const def = computed(() => ICONS[name])

const sizeClass = computed(() => ({
  xs: 'size-3.5',
  sm: 'size-4',
  md: 'size-5',
  lg: 'size-6',
  xl: 'size-8',
}[size]))
</script>
