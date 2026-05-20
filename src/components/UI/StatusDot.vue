<template>
  <span
    :class="[
      'inline-block size-2 rounded-full',
      pulse ? 'animate-pulse' : '',
      toneClass,
    ]"
  />
</template>

<script setup lang="ts">
import { computed } from 'vue'

type Tone = 'neutral' | 'success' | 'warning' | 'danger' | 'info' | 'accent'

const { tone = 'neutral', pulse = false } = defineProps<{
  tone?: Tone
  pulse?: boolean
}>()

const toneClass = computed(() => ({
  neutral: 'bg-slate-500',
  success: 'bg-emerald-400',
  warning: 'bg-amber-400',
  danger: 'bg-rose-400',
  info: 'bg-sky-400',
  accent: 'bg-pink-400',
}[tone]))
</script>
