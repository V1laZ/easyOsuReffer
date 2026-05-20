<template>
  <button
    :type="type"
    :disabled="disabled"
    :title="title"
    :class="[
      'inline-flex items-center justify-center rounded-lg transition-colors',
      'disabled:cursor-not-allowed disabled:opacity-50',
      'focus-visible:ring-2 focus-visible:ring-pink-400/60',
      sizeClass,
      variantClass,
    ]"
  >
    <Icon
      v-if="icon"
      :name="icon"
      :size="iconSize"
    />
    <slot v-else />
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Icon from './Icon.vue'
import type { IconName } from './icons'

type Variant = 'ghost' | 'accent' | 'danger'
type Size = 'sm' | 'md' | 'lg'

const {
  variant = 'ghost',
  size = 'md',
  type = 'button',
  disabled = false,
} = defineProps<{
  icon?: IconName
  variant?: Variant
  size?: Size
  type?: 'button' | 'submit' | 'reset'
  disabled?: boolean
  title?: string
}>()

const sizeClass = computed(() => ({
  sm: 'size-8',
  md: 'size-9',
  lg: 'size-10',
}[size]))

const iconSize = computed(() => ({
  sm: 'sm' as const,
  md: 'md' as const,
  lg: 'lg' as const,
}[size]))

const variantClass = computed(() => ({
  ghost: 'text-slate-400 hover:bg-slate-800 hover:text-slate-100',
  accent: 'text-pink-300 hover:bg-pink-500/15 hover:text-pink-200',
  danger: 'text-rose-300 hover:bg-rose-500/15 hover:text-rose-200',
}[variant]))
</script>
