<template>
  <label class="block space-y-1.5">
    <span
      v-if="label"
      class="block text-xs font-medium uppercase tracking-wide text-slate-400"
    >
      {{ label }}
      <span
        v-if="required"
        class="text-pink-300"
      >*</span>
    </span>
    <slot />
    <span
      v-if="hint"
      class="block text-xs text-slate-500"
    >{{ hint }}</span>
  </label>
</template>

<script setup lang="ts">
defineProps<{
  label?: string
  hint?: string
  required?: boolean
}>()
</script>
