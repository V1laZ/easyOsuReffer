<template>
  <span
    :class="[
      'inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium',
      variantClass,
    ]"
  >
    <slot />
  </span>
</template>

<script setup lang="ts">
import { computed } from 'vue'

type Tone = 'neutral' | 'accent' | 'success' | 'warning' | 'danger' | 'info'

const { tone = 'neutral' } = defineProps<{
  tone?: Tone
}>()

const variantClass = computed(() => ({
  neutral: 'bg-slate-800 text-slate-300 ring-1 ring-inset ring-slate-700',
  accent: 'bg-pink-500/15 text-pink-200 ring-1 ring-inset ring-pink-400/30',
  success: 'bg-emerald-500/15 text-emerald-200 ring-1 ring-inset ring-emerald-400/30',
  warning: 'bg-amber-500/15 text-amber-200 ring-1 ring-inset ring-amber-400/30',
  danger: 'bg-rose-500/15 text-rose-200 ring-1 ring-inset ring-rose-400/30',
  info: 'bg-sky-500/15 text-sky-200 ring-1 ring-inset ring-sky-400/30',
}[tone]))
</script>
