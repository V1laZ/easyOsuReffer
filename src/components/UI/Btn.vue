<template>
  <button
    :type="type"
    :disabled="disabled || loading"
    :class="[
      'inline-flex items-center justify-center gap-2 font-medium rounded-lg transition-colors',
      'disabled:cursor-not-allowed disabled:opacity-50',
      'focus-visible:ring-2 focus-visible:ring-pink-400/60',
      sizeClass,
      variantClass,
      block ? 'w-full' : '',
    ]"
  >
    <Spinner
      v-if="loading"
      class="size-4"
    />
    <slot
      v-else-if="$slots.icon"
      name="icon"
    />
    <span
      v-if="$slots.default"
      class="min-w-0 truncate"
    >
      <slot />
    </span>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Spinner from './Spinner.vue'

type Variant = 'primary' | 'secondary' | 'ghost' | 'danger' | 'success'
type Size = 'sm' | 'md' | 'lg'

const {
  variant = 'primary',
  size = 'md',
  type = 'button',
  block = false,
  loading = false,
  disabled = false,
} = defineProps<{
  variant?: Variant
  size?: Size
  type?: 'button' | 'submit' | 'reset'
  block?: boolean
  loading?: boolean
  disabled?: boolean
}>()

const sizeClass = computed(() => ({
  sm: 'h-8 px-3 text-xs',
  md: 'h-10 px-4 text-sm',
  lg: 'h-12 px-5 text-base',
}[size]))

const variantClass = computed(() => ({
  primary: 'bg-pink-500/15 text-pink-200 ring-1 ring-inset ring-pink-400/30 hover:bg-pink-500/25 hover:text-pink-100 hover:ring-pink-400/50',
  secondary: 'bg-slate-800 text-slate-200 ring-1 ring-inset ring-slate-700 hover:bg-slate-700 hover:text-white',
  ghost: 'text-slate-300 hover:bg-slate-800 hover:text-white',
  danger: 'bg-rose-500/15 text-rose-200 ring-1 ring-inset ring-rose-400/30 hover:bg-rose-500/25 hover:text-rose-100 hover:ring-rose-400/50',
  success: 'bg-emerald-500/15 text-emerald-200 ring-1 ring-inset ring-emerald-400/30 hover:bg-emerald-500/25 hover:text-emerald-100 hover:ring-emerald-400/50',
}[variant]))
</script>
