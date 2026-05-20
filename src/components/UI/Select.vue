<template>
  <div class="relative">
    <select
      v-model="model"
      :disabled="disabled"
      :class="[
        'w-full appearance-none rounded-lg bg-slate-800 px-3 py-2 pr-9 text-sm text-slate-100',
        'ring-1 ring-inset ring-slate-700',
        'focus:outline-none focus:ring-2 focus:ring-pink-400/60',
        'disabled:cursor-not-allowed disabled:opacity-50',
      ]"
    >
      <slot />
    </select>
    <svg
      class="pointer-events-none absolute right-3 top-1/2 size-3 -translate-y-1/2 text-slate-400"
      viewBox="0 0 12 8"
      fill="currentColor"
      aria-hidden="true"
    >
      <path d="M0 0h12L6 8z" />
    </svg>
  </div>
</template>

<script setup lang="ts" generic="T extends string | number | null">
const model = defineModel<T>({ required: true })

defineProps<{
  disabled?: boolean
}>()
</script>
