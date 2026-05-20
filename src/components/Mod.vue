<template>
  <span
    :class="[
      'inline-flex items-center rounded-md px-1.5 py-0.5 text-xs font-semibold ring-1 ring-inset',
      toneClass,
      active ? '' : 'opacity-40',
    ]"
  >
    {{ mod }}
  </span>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const { mod, active = true } = defineProps<{
  mod: string
  active?: boolean
}>()

const toneClass = computed(() => {
  switch (mod) {
    case 'DT':
    case 'NC':
      return 'bg-fuchsia-500/15 text-fuchsia-200 ring-fuchsia-400/30'
    case 'HD':
      return 'bg-amber-500/15 text-amber-200 ring-amber-400/30'
    case 'HR':
      return 'bg-rose-500/15 text-rose-200 ring-rose-400/30'
    case 'EZ':
      return 'bg-emerald-500/15 text-emerald-200 ring-emerald-400/30'
    case 'FL':
      return 'bg-slate-700 text-slate-200 ring-slate-600'
    case 'FM':
      return 'bg-pink-500/15 text-pink-200 ring-pink-400/30'
    case 'NF':
      return 'bg-slate-800 text-slate-300 ring-slate-700'
    case 'HT':
      return 'bg-sky-500/15 text-sky-200 ring-sky-400/30'
    default:
      return 'bg-slate-800 text-slate-300 ring-slate-700'
  }
})
</script>
